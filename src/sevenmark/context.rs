use crate::sevenmark::SevenMarkError;

/// 파싱 컨텍스트 - 재귀 깊이와 상태를 관리
///
/// 이 구조체는 파싱 과정에서 다음을 관리합니다:
/// - 재귀 깊이 제한을 통한 무한 재귀 방지
/// - 각주 중첩 방지
/// - 파싱 위치 추적을 위한 오프셋 관리
#[derive(Debug, Clone, Copy)]
pub struct ParseContext {
    /// 현재 재귀 깊이
    pub recursion_depth: usize,
    /// 각주 내부에서 파싱 중인지 여부 (중첩 각주 방지)
    pub in_footnote_context: bool,
    /// 최대 재귀 깊이
    pub max_recursion_depth: usize,
    /// 파싱 시작점의 기준 오프셋 (중첩 파싱 시 위치 계산용)
    pub base_offset: usize,
}

impl ParseContext {
    /// 새 컨텍스트 생성
    pub fn new() -> Self {
        Self {
            recursion_depth: 0,
            in_footnote_context: false,
            max_recursion_depth: 16, // 기존 MAX_RECURSION_DEPTH와 동일
            base_offset: 0,
        }
    }

    /// 재귀 깊이를 1 증가시키고 제한을 체크한 후 새로운 깊이 반환
    fn next_depth_checked(&self) -> Result<usize, SevenMarkError> {
        let new_depth = self.recursion_depth + 1;
        if new_depth > self.max_recursion_depth {
            return Err(SevenMarkError::RecursionDepthExceeded {
                depth: new_depth,
                max_depth: self.max_recursion_depth,
            });
        }
        Ok(new_depth)
    }

    /// 재귀 깊이를 1 증가시킨 새 컨텍스트 반환
    pub fn with_increased_depth(&self) -> Result<Self, SevenMarkError> {
        let new_depth = self.next_depth_checked()?;
        Ok(Self {
            recursion_depth: new_depth,
            ..*self
        })
    }

    /// 재귀 깊이를 1 감소시킨 새 컨텍스트 반환
    pub fn with_decreased_depth(&self) -> Result<Self, SevenMarkError> {
        if self.recursion_depth == 0 {
            return Err(SevenMarkError::RecursionDepthExceeded {
                depth: 0,
                max_depth: self.max_recursion_depth,
            });
        }
        Ok(Self {
            recursion_depth: self.recursion_depth - 1,
            ..*self
        })
    }

    /// 새 오프셋과 함께 재귀 깊이를 증가시킨 새 컨텍스트 반환
    pub fn with_offset(&self, offset: usize) -> Result<Self, SevenMarkError> {
        let new_depth = self.next_depth_checked()?;
        Ok(Self {
            recursion_depth: new_depth,
            in_footnote_context: self.in_footnote_context,
            max_recursion_depth: self.max_recursion_depth,
            base_offset: offset,
        })
    }

    /// 각주 컨텍스트로 전환한 새 컨텍스트 반환
    pub fn with_footnote_context(&self) -> Result<Self, SevenMarkError> {
        let new_depth = self.next_depth_checked()?;
        Ok(Self {
            recursion_depth: new_depth,
            in_footnote_context: true,
            max_recursion_depth: self.max_recursion_depth,
            base_offset: 0,
        })
    }

    pub fn without_footnote_context(&self) -> Self {
        Self {
            in_footnote_context: false,
            ..*self
        }
    }

    /// 현재 각주 컨텍스트에 있는지 확인
    pub fn is_in_footnote(&self) -> bool {
        self.in_footnote_context
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
}

impl Default for ParseContext {
    fn default() -> Self {
        Self::new()
    }
}
