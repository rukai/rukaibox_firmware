/// Simultaneous Opposing Cardinal Directions
/// https://www.hitboxarcade.com/blogs/support/what-is-socd
#[derive(Default)]
pub struct SocdState {
    pub prev_up: bool,
    pub prev_down: bool,
    pub prev_left: bool,
    pub prev_right: bool,
}

#[allow(dead_code)]
pub enum SocdType {
    SecondInputPriority,
    Neutral,
}

impl SocdType {
    pub fn resolve(
        &self,
        input1: bool,
        input2: bool,
        prev_input1: &mut bool,
        prev_input2: &mut bool,
    ) -> (bool, bool) {
        match self {
            SocdType::SecondInputPriority => {
                if input1 && input2 {
                    if *prev_input1 {
                        // pick input2
                        (false, true)
                    } else if *prev_input2 {
                        // pick input1
                        (true, false)
                    } else {
                        // tie break, just pick input1
                        (true, false)
                    }
                } else {
                    *prev_input1 = input1;
                    *prev_input2 = input2;
                    (input1, input2)
                }
            }
            SocdType::Neutral => {
                if input1 && input2 {
                    (false, false)
                } else {
                    (input1, input2)
                }
            }
        }
    }
}
