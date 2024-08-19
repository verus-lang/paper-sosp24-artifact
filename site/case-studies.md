---
layout: default
---

<div class="hero">
    <div class="main relative card header">
        <h1>Verus: A Practical Foundation for Systems Verification<br/>Case Studies</h1><a name="projects"/>
    </div>
    <div class="main relative card header">
        <ul class="publication">

        {% assign projects = site.projects | where: "type", "project" %}
        {% for project in projects %}
            <li class="case-studies">
                <div>
                  <a href="{{ project.code }}" class="paper-title">{{ project.title }}</a>
                </div>
                {{ project.content }}
            </li>
        {% endfor %}

        </ul>
    </div>
</div>
