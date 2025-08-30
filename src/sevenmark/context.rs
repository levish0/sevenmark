use crate::sevenmark::SevenMarkError;
use std::collections::HashSet;

macro_rules! context_setters {
    ($($name:ident => $field:ident),*) => {
        $(
            paste::paste! {
                /// [<$name:upper>] 컨텍스트로 전환
                pub fn [<set_ $name _context>](&mut self) {
                    self.$field = true;
                }

                /// [<$name:upper>] 컨텍스트 해제
                pub fn [<unset_ $name _context>](&mut self) {
                    self.$field = false;
                }
            }
        )*
    };
}

#[derive(Debug, Clone)]
pub struct ParseContext {
    pub recursion_depth: usize,
    pub inside_header: bool,
    pub inside_bold: bool,
    pub inside_italic: bool,
    pub inside_strikethrough: bool,
    pub inside_subscript: bool,
    pub inside_superscript: bool,
    pub inside_underline: bool,
    pub line_starts: HashSet<usize>,
    pub max_recursion_depth: usize,
}

impl ParseContext {
    /// 새 컨텍스트 생성
    pub fn new() -> Self {
        Self {
            recursion_depth: 0,
            inside_header: false,
            inside_bold: false,
            inside_italic: false,
            inside_strikethrough: false,
            inside_subscript: false,
            inside_superscript: false,
            inside_underline: false,
            line_starts: HashSet::new(),
            max_recursion_depth: 16,
        }
    }

    /// 현재 위치가 라인 시작인지 확인
    pub fn is_at_line_start(&self, position: usize) -> bool {
        self.line_starts.contains(&position)
    }

    /// 재귀 깊이 증가 (in-place)
    pub fn increase_depth(&mut self) -> Result<(), SevenMarkError> {
        let new_depth = self.recursion_depth + 1;
        if new_depth > self.max_recursion_depth {
            return Err(SevenMarkError::RecursionDepthExceeded {
                depth: new_depth,
                max_depth: self.max_recursion_depth,
            });
        }
        self.recursion_depth = new_depth;
        Ok(())
    }

    /// 재귀 깊이 감소 (in-place)
    pub fn decrease_depth(&mut self) -> Result<(), SevenMarkError> {
        if self.recursion_depth == 0 {
            return Err(SevenMarkError::RecursionDepthExceeded {
                depth: 0,
                max_depth: self.max_recursion_depth,
            });
        }
        self.recursion_depth -= 1;
        Ok(())
    }

    /// 최대 재귀 깊이에 도달했는지 확인
    pub fn is_at_max_depth(&self) -> bool {
        self.recursion_depth >= self.max_recursion_depth
    }

    /// 현재 재귀 깊이 반환
    pub fn current_depth(&self) -> usize {
        self.recursion_depth
    }

    /// 남은 재귀 깊이 반환
    pub fn remaining_depth(&self) -> usize {
        self.max_recursion_depth
            .saturating_sub(self.recursion_depth)
    }

    context_setters! {
        header => inside_header,
        bold => inside_bold,
        italic => inside_italic,
        strikethrough => inside_strikethrough,
        subscript => inside_subscript,
        superscript => inside_superscript,
        underline => inside_underline
    }
}

impl Default for ParseContext {
    fn default() -> Self {
        Self::new()
    }
}
