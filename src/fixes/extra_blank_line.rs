use super::Fix;
use crate::common::{LineEntry, LintKind};

pub(crate) struct ExtraBlankLineFixer {}

impl Default for ExtraBlankLineFixer {
    fn default() -> Self {
        Self {}
    }
}

impl Fix for ExtraBlankLineFixer {
    fn name(&self) -> LintKind {
        LintKind::ExtraBlankLine
    }

    fn fix_line(&mut self, line: &mut LineEntry) -> Option<()> {
        line.mark_as_deleted();
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{tests::line_entry, Warning};

    #[test]
    fn no_blank_lines_test() {
        let mut fixer = ExtraBlankLineFixer::default();

        let warnings = vec![];
        let lines = vec![
            line_entry(1, 3, "FOO=BAR"),
            line_entry(2, 3, ""),
            line_entry(3, 3, "HOGE=HUGA"),
        ];
        let mut fixing_lines = lines.clone();

        assert_eq!(Some(0), fixer.fix_warnings(warnings, &mut fixing_lines));
        assert_eq!(lines, fixing_lines);
    }

    #[test]
    fn fix_one_extra_blank_line_test() {
        let mut fixer = ExtraBlankLineFixer::default();

        let line1 = line_entry(1, 4, "FOO=BAR");
        let line2 = line_entry(2, 4, "");
        let line3 = line_entry(3, 4, "");
        let line4 = line_entry(4, 4, "HOGE=HUGA");
        let mut warning = Warning::new(
            line3.clone(),
            LintKind::ExtraBlankLine,
            "Extra blank line detected",
        );
        let warnings = vec![&mut warning];
        let mut lines = vec![line1, line2, line3, line4];
        assert_eq!(Some(1), fixer.fix_warnings(warnings, &mut lines));
    }

    #[test]
    fn fix_multiple_blank_lines_test() {
        let mut fixer = ExtraBlankLineFixer::default();

        let line1 = line_entry(1, 5, "FOO=BAR");
        let line2 = line_entry(2, 5, "");
        let line3 = line_entry(3, 5, "");
        let line4 = line_entry(4, 5, "");
        let line5 = line_entry(5, 5, "HOGE=HUGA");
        let mut warning1 = Warning::new(
            line3.clone(),
            LintKind::ExtraBlankLine,
            "Extra blank line detected",
        );
        let mut warning2 = Warning::new(
            line4.clone(),
            LintKind::ExtraBlankLine,
            "Extra blank line detected",
        );
        let warnings = vec![&mut warning1, &mut warning2];
        let mut lines = vec![line1, line2, line3, line4, line5];
        assert_eq!(Some(2), fixer.fix_warnings(warnings, &mut lines));
    }
}
