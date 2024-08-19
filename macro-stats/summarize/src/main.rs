use std::collections::HashMap;
#[allow(unused_braces)]
use std::path::{Path, PathBuf};

fn warn_p(project_id: &str, msg: &str) {
    eprintln!("warning: [{}] {}", project_id, msg);
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
struct VerificationResults {
    encountered_vir_error: bool,
    success: bool,
    verified: u32,
    errors: u32,
    is_verifying_entire_crate: bool,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
struct VerificationTimesMs {
    estimated_cpu_time: u64,
    total: u64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
struct VerificationOutput {
    verification_results: VerificationResults,
    times_ms: VerificationTimesMs,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
struct LineCountEntry {
    #[serde(default)]
    definitions: u64,
    #[serde(default)]
    proof: u64,
    #[serde(default)]
    layout: u64,
    #[serde(default)]
    comment: u64,
    #[serde(default)]
    trusted: u64,
    #[serde(default)]
    exec: u64,
    #[serde(default)]
    spec: u64,
    #[serde(default)]
    directives: u64,
    #[serde(rename = "proof,exec")]
    #[serde(default)]
    proof_exec: u64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
struct LineCountOutput {
    total: LineCountEntry,
}

fn parse_verification_output(path: &Path) -> VerificationOutput {
    let contents =
        std::fs::read_to_string(path).expect(&format!("failed to read {}", path.display()));
    serde_json::from_str(&contents)
        .unwrap_or_else(|err| panic!("cannot parse {}: {}", path.display(), err))
}

fn parse_line_count_output(path: &Path) -> Option<LineCountOutput> {
    let contents = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&contents).ok()
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
struct ModeSummaryVerus {
    wall_time_verus_s: f64,
    wall_time_s: f64,
    estimated_cpu_time_verus_s: f64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
struct ModeSummaryDafny {
    wall_time_s: f64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
struct ProjectSummaryVerus {
    project_id: String,
    success: bool,
    singlethread: ModeSummaryVerus,
    parallel: ModeSummaryVerus,
    linecount: LineCountSummaryVerus,
    encoding_size_mb: u64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
struct ProjectSummaryDafny {
    project_id: String,
    dafny_name: String,
    singlethread: ModeSummaryDafny,
    parallel: ModeSummaryDafny,
    linecount: LineCountSummaryDafny,
    encoding_size_mb: u64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
struct LineCountSummaryVerus {
    trusted: u64,
    proof: u64,
    exec: u64,
    both_proof_exec: u64,
    proof_exec_ratio: f32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
struct LineCountSummaryDafny {
    trusted: u64,
    proof: u64,
    exec: u64,
    proof_exec_ratio: f32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
struct ManualLineCount {
    trusted: u64,
    proof: u64,
    exec: u64,
    proof_code_ratio: f32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
struct VerusManualLineCountDelta {
    trusted: i64,
    proof: i64,
    exec: i64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
struct ManualProjectDatum {
    linecount: Option<ManualLineCount>,
    linecount_delta: Option<VerusManualLineCountDelta>,
    encoding_size_mbs: Option<u64>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
struct ManualProject {
    project_id: String,
    verus: Option<ManualProjectDatum>,
    dafny_baseline: Option<ManualProjectDatum>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
struct ProjectSummary {
    project_id: String,
    verus: ProjectSummaryVerus,
    dafny_baseline: Option<ProjectSummaryDafny>,
}

fn process_verus_project_time(
    project_id: &str,
    project_path: &Path,
    parallel: bool,
) -> (ModeSummaryVerus, bool) {
    let output = parse_verification_output(&project_path.join(format!(
        "verus-verification-{}.json",
        if parallel { "parallel" } else { "singlethread" }
    )));
    assert!(output.verification_results.is_verifying_entire_crate);
    if !output.verification_results.success {
        warn_p(project_id, "verification failed");
    }
    let time_contents = std::fs::read_to_string(project_path.join(format!(
        "verus-verification-{}.time.txt",
        if parallel { "parallel" } else { "singlethread" }
    )))
    .unwrap();
    let time_s = time_contents.parse::<f64>().unwrap();

    (
        ModeSummaryVerus {
            wall_time_verus_s: output.times_ms.total as f64 / 1000.0,
            estimated_cpu_time_verus_s: output.times_ms.estimated_cpu_time as f64 / 1000.0,
            wall_time_s: time_s,
        },
        output.verification_results.success,
    )
}

fn process_dafny_project_time(
    project_id: &str,
    project_path: &Path,
    dafny_name: &str,
    parallel: bool,
) -> ModeSummaryDafny {
    let time_contents = std::fs::read_to_string(project_path.join(format!(
        "{}-verification-{}.time.txt",
        dafny_name,
        if parallel { "parallel" } else { "singlethread" }
    )))
    .unwrap();
    let mut time_s = time_contents.parse::<f64>().unwrap();

    if project_id == "ironsht" {
        let time_nonlinear_contents = std::fs::read_to_string(project_path.join(format!(
            "{}-verification-{}-nonlinear.time.txt",
            dafny_name,
            if parallel { "parallel" } else { "singlethread" }
        )))
        .unwrap();
        time_s += time_nonlinear_contents.parse::<f64>().unwrap();
    }

    ModeSummaryDafny {
        wall_time_s: time_s,
    }
}

fn process_verus_project_line_count(
    _project_id: &str,
    project_path: &Path,
    delta: Option<&VerusManualLineCountDelta>,
) -> LineCountSummaryVerus {
    let output = parse_line_count_output(&project_path.join("verus-linecount.json"))
        .expect("line count missing");

    let trusted = output
        .total
        .trusted
        .saturating_add_signed(delta.map(|d| d.trusted).unwrap_or(0));
    let proof = (output.total.proof + output.total.spec + output.total.proof_exec)
        .saturating_add_signed(delta.map(|d| d.proof).unwrap_or(0));
    let exec = (output.total.exec + output.total.proof_exec)
        .saturating_add_signed(delta.map(|d| d.exec).unwrap_or(0));

    LineCountSummaryVerus {
        trusted,
        proof,
        exec,
        both_proof_exec: output.total.proof_exec,
        proof_exec_ratio: proof as f32 / exec as f32,
    }
}

fn process_verus_encoding_size(_project_id: &str, project_path: &Path) -> u64 {
    let encoding_tar = project_path.join("verus-encoding.tar.gz");

    let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
    use std::process::Command;

    let output = Command::new("tar")
        .arg("-xf")
        .arg(encoding_tar)
        .arg("-C")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute tar command");

    if !output.status.success() {
        panic!(
            "Failed to extract tar.gz file: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let output = Command::new("bash")
        .arg("encoding_bytes.sh")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute count command");
    if !output.status.success() {
        panic!(
            "Failed to execute count command: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let out = String::from_utf8_lossy(&output.stdout);
    out.lines()
        .next()
        .and_then(|l| l.trim().parse::<u64>().ok())
        .map(|v| (v as f64 / 1000.0 / 1000.0).round() as u64)
        .expect("encoding size")
}

fn process_verus_project(
    project_id: &str,
    project_path: &Path,
    manual: Option<&ManualProject>,
) -> ProjectSummaryVerus {
    let (singlethread, singlethread_success) =
        process_verus_project_time(project_id, project_path, false);
    let (parallel, parallel_success) = process_verus_project_time(project_id, project_path, true);
    let encoding_size_mb = process_verus_encoding_size(project_id, project_path);
    assert!(match &manual {
        Some(m) => {
            match &m.verus {
                Some(v) => v.linecount.is_none() && v.encoding_size_mbs.is_none(),
                None => true,
            }
        }
        None => true,
    });
    let manual_linecount_delta =
        manual.and_then(|m| m.verus.as_ref().and_then(|m| m.linecount_delta.as_ref()));
    ProjectSummaryVerus {
        project_id: project_id.to_owned(),
        singlethread,
        parallel,
        linecount: process_verus_project_line_count(
            project_id,
            project_path,
            manual_linecount_delta,
        ),
        success: singlethread_success && parallel_success,
        encoding_size_mb: encoding_size_mb,
    }
}

fn process_dafny_project(
    project_id: &str,
    project_path: &Path,
    dafny_name: &str,
    manual: Option<&ManualProject>,
) -> ProjectSummaryDafny {
    let singlethread = process_dafny_project_time(project_id, project_path, dafny_name, false);
    let parallel = process_dafny_project_time(project_id, project_path, dafny_name, true);
    let manual_linecount = manual
        .and_then(|m| m.dafny_baseline.as_ref())
        .and_then(|m| m.linecount.as_ref())
        .expect(&format!("linecount in dafny baseline for {project_id}"));
    let encoding_size_mb = *manual
        .and_then(|m| m.dafny_baseline.as_ref())
        .and_then(|m| m.encoding_size_mbs.as_ref())
        .expect(&format!("encoding size in dafny baseline for {project_id}"));
    ProjectSummaryDafny {
        project_id: project_id.to_owned(),
        dafny_name: dafny_name.to_owned(),
        singlethread,
        parallel,
        linecount: LineCountSummaryDafny {
            trusted: manual_linecount.trusted,
            proof: manual_linecount.proof,
            exec: manual_linecount.exec,
            proof_exec_ratio: manual_linecount.proof_code_ratio,
        },
        encoding_size_mb,
    }
}

const PROJECTS: &[(&str, Option<&str>)] = &[
    ("ironsht", Some("dafny")),
    ("nr", Some("linear-dafny")),
    ("page-table", None),
    ("mimalloc", None),
    ("verified-storage", None),
];

fn main() {
    let mut args = std::env::args();
    args.next().unwrap();

    let results = PathBuf::from(args.next().unwrap());

    let manual: HashMap<String, ManualProject> = {
        let s = match std::fs::read_to_string(&PathBuf::from("manual.json")) {
            Ok(v) => Some(v),
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => None,
                _ => panic!("failed to read manual.json: {}", e),
            },
        };
        let manual: Option<Vec<ManualProject>> =
            s.and_then(|x| serde_json::from_str(&x).expect("failed to parse manual.json"));
        manual
            .map(|mps| {
                mps.into_iter()
                    .map(|mp| (mp.project_id.clone(), mp))
                    .collect()
            })
            .unwrap_or(HashMap::new())
    };
    dbg!(&manual);

    let json_out_file = results.join("results.json");
    let latex_commands_out_file = results.join("results-latex-commands.tex");
    // let latex_table_out_file = results.join("results-latex-table.tex");

    let num_threads = {
        let verus_num_threads = std::fs::read_to_string(results.join("verus-num-threads.txt"))
            .unwrap()
            .trim()
            .parse::<u32>()
            .unwrap();
        for (_, dafny_baseline) in PROJECTS.iter() {
            if let Some(dafny_name) = dafny_baseline {
                let dafny_num_threads = std::fs::read_to_string(
                    results.join(format!("{}-num-threads.txt", dafny_name)),
                )
                .unwrap()
                .trim()
                .parse::<u32>()
                .unwrap();
                assert_eq!(dafny_num_threads, verus_num_threads);
            }
        }
        verus_num_threads
    };

    let summaries = PROJECTS
        .iter()
        .map(|(project, dafny_baseline)| {
            let s = ProjectSummary {
                project_id: (*project).to_owned(),
                verus: process_verus_project(project, &results.join(project), manual.get(*project)),
                dafny_baseline: dafny_baseline.map(|dafny_name| {
                    process_dafny_project(
                        project,
                        &results.join(project),
                        dafny_name,
                        manual.get(*project),
                    )
                }),
            };
            s
        })
        .collect::<Vec<_>>();

    {
        let mut json_out = std::fs::File::create(json_out_file).unwrap();
        serde_json::to_writer_pretty(&mut json_out, &summaries).unwrap();
    }

    fn project_id_name(project_id: &str) -> String {
        project_id
            .split('-')
            .map(|s| {
                s.chars()
                    .next()
                    .unwrap()
                    .to_uppercase()
                    .chain(s.chars().skip(1))
                    .collect::<String>()
            })
            .collect::<String>()
    }

    {
        fn emit_commands_verus(
            summary: &ProjectSummaryVerus,
            latex_commands_out: &mut std::fs::File,
            project_id_name: &str,
        ) {
            writeln!(
                latex_commands_out,
                "\\newcommand{{\\evalVerus{}Success}}{{{}}}",
                project_id_name, summary.success
            )
            .unwrap();
            writeln!(
                latex_commands_out,
                "\\newcommand{{\\evalVerus{}SinglethreadWallTime}}{{{:.0}}}",
                project_id_name, summary.singlethread.wall_time_s
            )
            .unwrap();
            writeln!(
                latex_commands_out,
                "\\newcommand{{\\evalVerus{}ParallelWallTime}}{{{:.0}}}",
                project_id_name, summary.parallel.wall_time_s
            )
            .unwrap();
            writeln!(
                latex_commands_out,
                "\\newcommand{{\\evalVerus{}LineCountTrusted}}{{{}}}",
                project_id_name, summary.linecount.trusted
            )
            .unwrap();
            writeln!(
                latex_commands_out,
                "\\newcommand{{\\evalVerus{}LineCountProof}}{{{}}}",
                project_id_name, summary.linecount.proof
            )
            .unwrap();
            writeln!(
                latex_commands_out,
                "\\newcommand{{\\evalVerus{}LineCountExec}}{{{}}}",
                project_id_name, summary.linecount.exec
            )
            .unwrap();
            writeln!(
                latex_commands_out,
                "\\newcommand{{\\evalVerus{}LineCountProofCodeRatio}}{{{:.1}}}",
                project_id_name, summary.linecount.proof_exec_ratio
            )
            .unwrap();
            writeln!(
                latex_commands_out,
                "\\newcommand{{\\evalVerus{}EncodingSizeMB}}{{{}}}",
                project_id_name, summary.encoding_size_mb,
            )
            .unwrap();
        }

        fn emit_commands_dafny_baseline(
            summary: &ProjectSummaryDafny,
            latex_commands_out: &mut std::fs::File,
            dafny_name: &str,
            project_id_name: &str,
        ) {
            writeln!(
                latex_commands_out,
                "\\newcommand{{\\eval{}{}SinglethreadWallTime}}{{{:.0}}}",
                dafny_name, project_id_name, summary.singlethread.wall_time_s
            )
            .unwrap();
            writeln!(
                latex_commands_out,
                "\\newcommand{{\\eval{}{}ParallelWallTime}}{{{:.0}}}",
                dafny_name, project_id_name, summary.parallel.wall_time_s
            )
            .unwrap();
            writeln!(
                latex_commands_out,
                "\\newcommand{{\\eval{}{}LineCountTrusted}}{{{}}}",
                dafny_name, project_id_name, summary.linecount.trusted
            )
            .unwrap();
            writeln!(
                latex_commands_out,
                "\\newcommand{{\\eval{}{}LineCountProof}}{{{}}}",
                dafny_name, project_id_name, summary.linecount.proof
            )
            .unwrap();
            writeln!(
                latex_commands_out,
                "\\newcommand{{\\eval{}{}LineCountExec}}{{{}}}",
                dafny_name, project_id_name, summary.linecount.exec
            )
            .unwrap();
            writeln!(
                latex_commands_out,
                "\\newcommand{{\\eval{}{}LineCountProofCodeRatio}}{{{:.1}}}",
                dafny_name, project_id_name, summary.linecount.proof_exec_ratio
            )
            .unwrap();
            writeln!(
                latex_commands_out,
                "\\newcommand{{\\eval{}{}EncodingSizeMB}}{{{}}}",
                dafny_name, project_id_name, summary.encoding_size_mb,
            )
            .unwrap();
        }

        let mut latex_commands_out = std::fs::File::create(latex_commands_out_file).unwrap();
        use std::io::Write;
        writeln!(
            latex_commands_out,
            "\\newcommand{{\\evalVerusProjectCount}}{{{}}}",
            summaries.len()
        )
        .unwrap();
        writeln!(
            latex_commands_out,
            "\\newcommand{{\\evalParallelNumThreads}}{{{}}}",
            num_threads
        )
        .unwrap();
        for summary in summaries.iter() {
            let project_id_name = project_id_name(&summary.project_id);

            emit_commands_verus(&summary.verus, &mut latex_commands_out, &project_id_name);

            if let Some(dafny_baseline) = &summary.dafny_baseline {
                let dafny_name = dafny_baseline
                    .dafny_name
                    .split('-')
                    .map(|s| {
                        s.chars()
                            .next()
                            .unwrap()
                            .to_uppercase()
                            .chain(s.chars().skip(1))
                            .collect::<String>()
                    })
                    .collect::<String>();
                emit_commands_dafny_baseline(
                    dafny_baseline,
                    &mut latex_commands_out,
                    &dafny_name,
                    &project_id_name,
                );
            }
        }

        struct Totals {
            trusted: u64,
            proof: u64,
            exec: u64,
        }
        let mut t = Totals {
            trusted: 0,
            proof: 0,
            exec: 0,
        };
        for summary in summaries.iter() {
            let linecount = &summary.verus.linecount;
            t.trusted += linecount.trusted;
            t.proof += linecount.proof;
            t.exec += linecount.exec;
        }
        let verus_total_proof_exec_ratio = t.proof as f32 / t.exec as f32;
        writeln!(
            latex_commands_out,
            "\\newcommand{{\\evalVerusTotalLinesTrusted}}{{{:.1}K}}",
            t.trusted as f64 / 1000.0
        )
        .unwrap();
        writeln!(
            latex_commands_out,
            "\\newcommand{{\\evalVerusTotalLinesProof}}{{{:.1}K}}",
            t.proof as f64 / 1000.0
        )
        .unwrap();
        writeln!(
            latex_commands_out,
            "\\newcommand{{\\evalVerusTotalLinesExec}}{{{:.1}K}}",
            t.exec as f64 / 1000.0
        )
        .unwrap();
        writeln!(
            latex_commands_out,
            "\\newcommand{{\\evalVerusTotalLinesProofCodeRatio}}{{{:.1}}}",
            verus_total_proof_exec_ratio
        )
        .unwrap();
    }
}
