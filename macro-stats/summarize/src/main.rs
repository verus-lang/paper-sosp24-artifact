#[allow(unused_braces)]
use std::{
    path::{Path, PathBuf},
};

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
    // eprintln!("{}: {}", path.display(), contents);
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
    linecount: Option<LineCountSummary>,
    encoding_size_mb: Option<u64>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
struct ProjectSummaryDafny {
    project_id: String,
    dafny_name: String,
    singlethread: ModeSummaryDafny,
    parallel: ModeSummaryDafny,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
struct LineCountSummary {
    trusted: u64,
    proof: u64,
    exec: u64,
    both_proof_exec: u64,
    proof_exec_ratio: f32,
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
) -> Option<LineCountSummary> {
    let output = parse_line_count_output(&project_path.join("verus-linecount.json"))?;

    let proof = output.total.proof + output.total.spec + output.total.proof_exec;
    let exec = output.total.exec + output.total.proof_exec;

    Some(LineCountSummary {
        trusted: output.total.trusted,
        proof,
        exec,
        both_proof_exec: output.total.proof_exec,
        proof_exec_ratio: proof as f32 / exec as f32,
    })
}

fn process_verus_encoding_size(_project_id: &str, project_path: &Path) -> Option<u64> {
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
    
    // dbg!(&String::from_utf8_lossy(&output.stderr), &String::from_utf8_lossy(&output.stdout));
    // dbg!(&temp_dir.path());

    if !output.status.success() {
        panic!(
            "Failed to extract tar.gz file: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    // let dir_name = {
    //     let mut paths = std::fs::read_dir(temp_dir.path()).expect("the tar output is unexpected");
    //     let dir_name = paths.next().expect("one directory in the tar file")
    //         .expect("valid fs call");
    //     assert!(paths.next().is_none());
    //     dir_name.file_name()
    // };

    // let _ = std::io::Read::read(&mut std::io::stdin(), &mut [0u8]).unwrap();

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
    // let err = String::from_utf8_lossy(&output.stderr);
    // dbg!(&out, &err);
    out.lines().next().and_then(|l| l.trim().parse::<u64>().ok()).map(|v| (v as f64 / 1000.0 / 1000.0).round() as u64)
}

fn process_verus_project(project_id: &str, project_path: &Path) -> ProjectSummaryVerus {
    let (singlethread, singlethread_success) =
        process_verus_project_time(project_id, project_path, false);
    let (parallel, parallel_success) = process_verus_project_time(project_id, project_path, true);
    let encoding_size_mb = process_verus_encoding_size(project_id, project_path);
    ProjectSummaryVerus {
        project_id: project_id.to_owned(),
        singlethread,
        parallel,
        linecount: process_verus_project_line_count(project_id, project_path),
        success: singlethread_success && parallel_success,
        encoding_size_mb: encoding_size_mb,
    }
}

fn process_dafny_project(
    project_id: &str,
    project_path: &Path,
    dafny_name: &str,
) -> ProjectSummaryDafny {
    let singlethread = process_dafny_project_time(project_id, project_path, dafny_name, false);
    let parallel = process_dafny_project_time(project_id, project_path, dafny_name, true);
    ProjectSummaryDafny {
        project_id: project_id.to_owned(),
        dafny_name: dafny_name.to_owned(),
        singlethread,
        parallel,
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
    // let encodings_tar = args.next();

    let json_out_file = results.join("results.json");
    let latex_commands_out_file = results.join("results-latex-commands.tex");
    let latex_table_out_file = results.join("results-latex-table.tex");

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
                verus: process_verus_project(project, &results.join(project)),
                dafny_baseline: dafny_baseline.map(|dafny_name| {
                    process_dafny_project(project, &results.join(project), dafny_name)
                }),
            };
            // if let Some(project_verus_encodings_mbs) = &project_verus_encodings_mbs {
            //     s.verus.encoding_size_mb = Some(project_verus_encodings_mbs[*project]);
            // }
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

    #[cfg(old)]
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
                "\\newcommand{{\\evalVerus{}SinglethreadWallTime}}{{{:.2}}}",
                project_id_name, summary.singlethread.wall_time_s
            )
            .unwrap();
            writeln!(
                latex_commands_out,
                "\\newcommand{{\\evalVerus{}ParallelWallTime}}{{{:.2}}}",
                project_id_name, summary.parallel.wall_time_s
            )
            .unwrap();
            if let Some(linecount) = &summary.linecount {
                writeln!(
                    latex_commands_out,
                    "\\newcommand{{\\evalVerus{}LineCountTrusted}}{{{}}}",
                    project_id_name, linecount.trusted
                )
                .unwrap();
                writeln!(
                    latex_commands_out,
                    "\\newcommand{{\\evalVerus{}LineCountProof}}{{{}}}",
                    project_id_name, linecount.proof
                )
                .unwrap();
                writeln!(
                    latex_commands_out,
                    "\\newcommand{{\\evalVerus{}LineCountExec}}{{{}}}",
                    project_id_name, linecount.exec
                )
                .unwrap();
                writeln!(
                    latex_commands_out,
                    "\\newcommand{{\\evalVerus{}LineCountProofCodeRatio}}{{{:.2}}}",
                    project_id_name, linecount.proof_exec_ratio
                )
                .unwrap();
            } else {
                writeln!(
                    latex_commands_out,
                    "\\newcommand{{\\evalVerus{}LineCountTrusted}}{{TODO}}",
                    project_id_name
                )
                .unwrap();
                writeln!(
                    latex_commands_out,
                    "\\newcommand{{\\evalVerus{}LineCountProof}}{{TODO}}",
                    project_id_name
                )
                .unwrap();
                writeln!(
                    latex_commands_out,
                    "\\newcommand{{\\evalVerus{}LineCountExec}}{{TODO}}",
                    project_id_name
                )
                .unwrap();
                writeln!(
                    latex_commands_out,
                    "\\newcommand{{\\evalVerus{}LineCountProofCodeRatio}}{{TODO}}",
                    project_id_name
                )
                .unwrap();
            }
        }

        fn emit_commands_dafny_baseline(
            summary: &ProjectSummaryDafny,
            latex_commands_out: &mut std::fs::File,
            dafny_name: &str,
            project_id_name: &str,
        ) {
            writeln!(
                latex_commands_out,
                "\\newcommand{{\\eval{}{}SinglethreadWallTime}}{{{:.2}}}",
                dafny_name, project_id_name, summary.singlethread.wall_time_s
            )
            .unwrap();
            writeln!(
                latex_commands_out,
                "\\newcommand{{\\eval{}{}ParallelWallTime}}{{{:.2}}}",
                dafny_name, project_id_name, summary.parallel.wall_time_s
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
                // dafny name from "linear-dafny" to "LinearDafny"
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
            if let Some(linecount) = &summary.linecount {
                writeln!(
                    latex_commands_out,
                    "\\newcommand{{\\evalVerus{}LineCountTrusted}}{{{}}}",
                    project_id_name, linecount.trusted
                )
                .unwrap();
                writeln!(
                    latex_commands_out,
                    "\\newcommand{{\\evalVerus{}LineCountProof}}{{{}}}",
                    project_id_name, linecount.proof
                )
                .unwrap();
                writeln!(
                    latex_commands_out,
                    "\\newcommand{{\\evalVerus{}LineCountExec}}{{{}}}",
                    project_id_name, linecount.exec
                )
                .unwrap();
                writeln!(
                    latex_commands_out,
                    "\\newcommand{{\\evalVerus{}LineCountProofCodeRatio}}{{{:.1}}}",
                    project_id_name, linecount.proof_exec_ratio
                )
                .unwrap();
            } else {
                writeln!(
                    latex_commands_out,
                    "\\newcommand{{\\evalVerus{}LineCountTrusted}}{{TODO}}",
                    project_id_name
                )
                .unwrap();
                writeln!(
                    latex_commands_out,
                    "\\newcommand{{\\evalVerus{}LineCountProof}}{{TODO}}",
                    project_id_name
                )
                .unwrap();
                writeln!(
                    latex_commands_out,
                    "\\newcommand{{\\evalVerus{}LineCountExec}}{{TODO}}",
                    project_id_name
                )
                .unwrap();
                writeln!(
                    latex_commands_out,
                    "\\newcommand{{\\evalVerus{}LineCountProofCodeRatio}}{{TODO}}",
                    project_id_name
                )
                .unwrap();
            }
            if let Some(encoding_mb) = &summary.encoding_size_mb {
                writeln!(
                    latex_commands_out,
                    "\\newcommand{{\\evalVerus{}EncodingSizeMB}}{{{}}}",
                    project_id_name, encoding_mb
                )
                .unwrap();
            } else {
                writeln!(
                    latex_commands_out,
                    "\\newcommand{{\\evalVerus{}EncodingSizeMB}}{{TODO}}",
                    project_id_name
                )
                .unwrap();
            }
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
                // dafny name from "linear-dafny" to "LinearDafny"
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

        // struct Totals { proof: u64, exec: u64, both_proof_exec: u64 }
        // let mut t = Totals { proof: 0, exec: 0, both_proof_exec: 0 };
        // for summary in summaries.iter() {
        //     let Some(linecount) = &summary.verus.linecount else { panic!("missing line count") };
        //     dbg!(&summary);
        //     t.proof += linecount.proof;
        //     t.exec += linecount.exec;
        //     t.both_proof_exec += linecount.both_proof_exec;
        // }
        // writeln!(latex_commands_out, "\\newcommand{{\\evalVerusTotalProofLines}}{{{:.1}K}}", t.proof as f64 / 1000.0).unwrap();
        // writeln!(latex_commands_out, "\\newcommand{{\\evalVerusTotalExecLines}}{{{:.1}K}}", t.exec as f64 / 1000.0).unwrap();
        // writeln!(latex_commands_out, "\\newcommand{{\\evalVerusTotalProofExecLines}}{{{:.1}}}", t.both_proof_exec).unwrap();
    }

    {
        let mut latex_table_out = std::fs::File::create(latex_table_out_file).unwrap();
        use std::io::Write;

        writeln!(latex_table_out, "\\documentclass{{article}}").unwrap();

        writeln!(latex_table_out, "\\input{{results-latex-commands}}").unwrap();

        writeln!(latex_table_out, "\\begin{{document}}").unwrap();

        writeln!(latex_table_out, "\\begin{{tabular}}{{l|c|c|c|c|c|c}}").unwrap();

        writeln!(latex_table_out, "Project & \\multicolumn{{2}}{{c|}}{{Time (s)}} & \\multicolumn{{3}}{{c|}}{{Line Count}} \\\\").unwrap();
        writeln!(
            latex_table_out,
            " & 1 thread & \\evalParallelNumThreads threads & trusted & proof & exec \\\\"
        )
        .unwrap();
        writeln!(latex_table_out, "\\hline").unwrap();
        for summary in summaries.iter() {
            let project_id_name = project_id_name(&summary.project_id);
            writeln!(latex_table_out, "{} & \\evalVerus{}SinglethreadWallTime & \\evalVerus{}ParallelWallTime & \\evalVerus{}LineCountTrusted & \\evalVerus{}LineCountProof & \\evalVerus{}LineCountExec \\\\", project_id_name, project_id_name, project_id_name, project_id_name, project_id_name, project_id_name).unwrap();
        }
        writeln!(latex_table_out, "\\end{{tabular}}").unwrap();

        writeln!(latex_table_out, "\\end{{document}}").unwrap();
    }
}
