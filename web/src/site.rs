//! Site facts: name, contact, project entries. Verifiable facts only, no self-description.

use crate::model::{Project, Site};

pub static SITE: Site = Site {
    name: "Liu Jiajun",
    name_latin: "@Jiajun0413",
    meta_line: "Code, tools, and notes.",
    github: "https://github.com/Jiajun0413",
    orcid: "https://orcid.org/0009-0004-4258-2237",
    email: "jiajun_liu0805@foxmail.com",
};

pub static PROJECTS: &[Project] = &[
    Project {
        name: "oh-my-opencode-slim",
        note: "10k+ stars · Collaborator",
        summary: "Multi-agent orchestration for OpenCode.",
        href: "https://github.com/alvinunreal/oh-my-opencode-slim",
        link_label: None,
    },
    Project {
        name: "pixi-skill",
        note: "Skill",
        summary: "Pixi manifests, environments, tasks, and global tools.",
        href: "https://github.com/Jiajun0413/pixi-skill",
        link_label: None,
    },
    Project {
        name: "omo-watcher",
        note: "Plugin",
        summary: "Image hooks for oh-my-openagent.",
        href: "https://github.com/Jiajun0413/omo-watcher",
        link_label: None,
    },
    Project {
        name: "pdf2zh",
        note: "Password required",
        summary: "Translate papers while preserving formulas and layout.",
        href: "/translate/",
        link_label: Some("Translate a paper"),
    },
    Project {
        name: "MoM-ISEP",
        note: "Platform",
        summary: "Protein immune-safety database and prediction for allergens, autoantigens, and B-cell epitopes.",
        href: "https://pisd.cc.cd/",
        link_label: Some("pisd.cc.cd"),
    },
    Project {
        name: "Shanghai Bioinformatics Data Sharing",
        note: "Platform",
        summary: "Bioinformatics data search and sharing.",
        href: "https://sbdssp.cc.cd/",
        link_label: Some("sbdssp.cc.cd"),
    },
];
