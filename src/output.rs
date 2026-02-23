use console::{Term, style};
use std::fmt::Display;

/// 终端输出工具
pub struct Output {
    term: Term,
    #[allow(dead_code)]
    is_stderr: bool,
}

impl Default for Output {
    fn default() -> Self {
        Self {
            term: Term::stdout(),
            is_stderr: false,
        }
    }
}

impl Output {
    /// 创建新的标准错误输出实例
    pub fn new_stderr() -> Self {
        Self {
            term: Term::stderr(),
            // 标记此输出实例是否写入标准错误流
            is_stderr: true,
        }
    }

    /// 输出普通信息
    pub fn info<D: Display>(&self, message: D) {
        self.term.write_line(&message.to_string()).unwrap();
    }

    /// 输出成功信息（绿色）
    pub fn success<D: Display>(&self, message: D) {
        self.term
            .write_line(&style(message).green().to_string())
            .unwrap();
    }

    /// 输出警告信息（黄色）
    pub fn warning<D: Display>(&self, message: D) {
        self.term
            .write_line(&style(message).yellow().to_string())
            .unwrap();
    }

    /// 输出错误信息（红色）
    pub fn error<D: Display>(&self, message: D) {
        self.term
            .write_line(&style(message).red().to_string())
            .unwrap();
    }

    /// 输出调试信息（灰色）
    pub fn debug<D: Display>(&self, message: D) {
        self.term
            .write_line(&style(message).dim().to_string())
            .unwrap();
    }

    /// 输出强调信息（粗体）
    pub fn bold<D: Display>(&self, message: D) {
        self.term
            .write_line(&style(message).bold().to_string())
            .unwrap();
    }

    /// 输出标题（粗体+下划线）
    pub fn title<D: Display>(&self, message: D) {
        self.term
            .write_line(&style(message).bold().underlined().to_string())
            .unwrap();
    }

    /// 输出调试表格
    pub fn debug_table(&self, title: &str, rows: &[(&str, &dyn std::fmt::Display)]) {
        // 计算列宽
        let left_width = rows.iter().map(|(left, _)| left.len()).max().unwrap_or(0) + 2;
        let right_width = rows
            .iter()
            .map(|(_, right)| format!("{right}").len())
            .max()
            .unwrap_or(0)
            + 2;

        // 生成表格
        self.debug(format!(
            "  ┌{}{}┐",
            "─".repeat(left_width + 2),
            "─".repeat(right_width + 2)
        ));
        self.debug(format!(
            "  │ {:<width$}│",
            title,
            width = left_width + right_width + 2
        ));
        self.debug(format!(
            "  ├{}{}┼{}{}┤",
            "─".repeat(left_width),
            "─".repeat(3),
            "─".repeat(right_width),
            "─".repeat(3)
        ));

        for (left, right) in rows {
            self.debug(format!("  │ {left:<left_width$} │ {right:<right_width$} │"));
        }

        self.debug(format!(
            "  └{}{}┴{}{}┘",
            "─".repeat(left_width),
            "─".repeat(3),
            "─".repeat(right_width),
            "─".repeat(3)
        ));
    }

    /// 输出章节列表
    pub fn chapter_list(&self, chapters: &[(usize, &str, usize, usize)]) {
        self.title("Detected Chapters");
        self.info("\n");

        for (i, title, start_line, end_line) in chapters {
            let line_range = format!("lines {}-{}", start_line + 1, end_line + 1);
            let formatted = format!(
                "[{:03}] {} ({})
",
                i + 1,
                style(title).bold(),
                style(line_range).dim()
            );
            self.term.write_line(&formatted).unwrap();
        }

        self.info("\n");
    }

    /// 输出章节大纲
    pub fn chapter_outline(&self, chapters: &[&str]) {
        self.title("Chapter Outline");
        self.info("\n");

        for (i, title) in chapters.iter().enumerate() {
            let formatted = format!(
                "{}. {}
",
                i + 1,
                title
            );
            self.term.write_line(&formatted).unwrap();
        }

        self.info("\n");
    }

    /// 输出性能摘要
    pub fn performance_summary(&self, title: &str, metrics: &[(&str, std::time::Duration)]) {
        self.bold(title);
        self.info("\n");

        for (name, duration) in metrics {
            let formatted = format!("{}: {:.2?}", style(name).bold(), duration);
            self.term.write_line(&formatted).unwrap();
        }

        self.info("\n");
    }

    /// 输出分隔线
    pub fn separator(&self) {
        self.info("─".repeat(80));
    }

    /// 输出空行
    pub fn empty_line(&self) {
        self.info("");
    }
}

/// 创建默认输出实例
pub fn new_output() -> Output {
    Output::default()
}

// 全局输出实例（用于简单调用）
lazy_static::lazy_static! {
    pub static ref GLOBAL_OUTPUT: Output = Output::default();
    pub static ref GLOBAL_ERROR_OUTPUT: Output = Output::new_stderr();
}
