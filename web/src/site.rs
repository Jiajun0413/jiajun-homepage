//! 站点事实：姓名、联系方式、项目条目。只放可核查的事实，不放自我评价。

use crate::model::{Project, Site};

pub static SITE: Site = Site {
    name: "Liu Jiajun",
    name_latin: "@Jiajun0413",
    meta_line: "代码、工具与阅读笔记",
    github: "https://github.com/Jiajun0413",
    email: "jiajun_liu0805@foxmail.com",
};

pub static PROJECTS: &[Project] = &[
    Project {
        name: "oh-my-opencode-slim",
        note: "10k+ ★ · 主要贡献者",
        summary: "OpenCode 多智能体编排插件。参与开发与维护，让不同模型协作完成任务。",
        href: "https://github.com/alvinunreal/oh-my-opencode-slim",
        link_label: None,
    },
    Project {
        name: "omo-watcher",
        note: "插件",
        summary: "为 oh-my-openagent 写的图片钩子插件。",
        href: "https://github.com/Jiajun0413/omo-watcher",
        link_label: None,
    },
    Project {
        name: "pdf2zh",
        note: "在线服务",
        summary: "论文 PDF 翻译成中文，保留公式、图表与排版。",
        href: "/translate/",
        link_label: Some("打开翻译"),
    },
    Project {
        name: "MoM-ISEP",
        note: "平台",
        summary: "蛋白免疫安全数据库与过敏原、自身抗原和 B 细胞表位预测工具。",
        href: "https://pisd.cc.cd/",
        link_label: Some("pisd.cc.cd"),
    },
    Project {
        name: "上海市生物信息数据共享服务平台",
        note: "平台",
        summary: "生物信息数据检索与共享服务。",
        href: "https://sbdssp.cc.cd/",
        link_label: Some("sbdssp.cc.cd"),
    },
];
