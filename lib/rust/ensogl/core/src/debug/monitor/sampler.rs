//! Definition of performance monitor sampler and a set of predefined samplers. Sampler is a utility
//! capable of displaying a single performance metric, like the current FPS or number of draw calls
//! per frame.

use crate::prelude::*;

use crate::debug::stats::StatsData;

use num_traits::cast::AsPrimitive;



// ==================
// === ValueCheck ===
// ==================

/// Values drawn in the monitor can be assigned with a check: `Correct`, `Warning`, and `Error`.
/// It affects the way they are visually displayed.
#[derive(Copy, Clone, Debug)]
#[allow(missing_docs)]
pub enum ValueCheck {
    Correct,
    Warning,
    Error,
}

impl Default for ValueCheck {
    fn default() -> Self {
        Self::Correct
    }
}

// To be removed after this gets resolved: https://github.com/rust-lang/rust-clippy/issues/4971
#[allow(clippy::collapsible_else_if)]
impl ValueCheck {
    /// Construct the check by comparing the provided value to two threshold values.
    pub fn from_threshold(warn_threshold: f64, err_threshold: f64, value: f64) -> Self {
        if warn_threshold > err_threshold {
            if value >= warn_threshold {
                ValueCheck::Correct
            } else if value >= err_threshold {
                ValueCheck::Warning
            } else {
                ValueCheck::Error
            }
        } else {
            if value <= warn_threshold {
                ValueCheck::Correct
            } else if value <= err_threshold {
                ValueCheck::Warning
            } else {
                ValueCheck::Error
            }
        }
    }
}



// ===============
// === Sampler ===
// ===============

/// Sampler is an utility to gather performance-related data and expose it in a way understandable
/// by the performance monitor.
#[derive(Copy, Clone)]
pub struct Sampler {
    /// Label of the sampler to be displayed in the performance monitor window.
    pub label:          &'static str,
    /// Get the newest value of the sampler. The value will be displayed in the monitor panel.
    pub expr:           fn(&StatsData) -> f64,
    /// Get the details to be displayed in the details view.
    pub details:        Option<fn(&StatsData) -> &[&'static str]>,
    /// If the value crosses this threshold, the graph will be drawn in the warning color.
    pub warn_threshold: f64,
    /// If the value crosses this threshold, the graph will be drawn in the error color.
    pub err_threshold:  f64,
    /// The value will be divided by this number before being displayed.
    pub value_divisor:  f64,
    /// The minimum expected value in order to set proper scaling of the monitor plots. If the real
    /// value will be smaller than this parameter, it will be clamped.
    pub min_value:      Option<f64>,
    /// The maximum expected value in order to set proper scaling of the monitor plots. If the real
    /// value will be bigger than this parameter, it will be clamped.
    pub max_value:      Option<f64>,
    /// The number of digits after the dot which should be displayed in the monitor panel.
    pub precision:      usize,
}

impl Debug for Sampler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sampler")
    }
}

impl const Default for Sampler {
    fn default() -> Self {
        Self {
            label:          "Unlabeled",
            expr:           |_| 0.0,
            details:        None,
            warn_threshold: 0.0,
            err_threshold:  0.0,
            value_divisor:  1.0,
            min_value:      None,
            max_value:      None,
            precision:      0,
        }
    }
}

impl Sampler {
    /// The current sampler value.
    pub fn value(&self, stats: &StatsData) -> f64 {
        let raw_value: f64 = (self.expr)(stats).as_();
        raw_value / self.value_divisor
    }

    /// Check the current value in order to draw it with warning or error if it exceeds the allowed
    /// thresholds.
    pub fn check(&self, stats: &StatsData) -> ValueCheck {
        let value = self.value(stats);
        ValueCheck::from_threshold(self.warn_threshold, self.err_threshold, value)
    }

    /// Minimum size of the size the sampler should occupy in the performance monitor view.
    pub fn min_size(&self) -> Option<f64> {
        Some(self.warn_threshold)
    }
}



// ================
// === Samplers ===
// ================

const MB: f64 = (1024 * 1024) as f64;

const DEFAULT_SAMPLER: Sampler = Default::default();

#[allow(missing_docs)]
pub const FPS: Sampler = Sampler {
    label: "Frames per second",
    expr: |s| s.fps,
    warn_threshold: 55.0,
    err_threshold: 25.0,
    precision: 2,
    max_value: Some(60.0),
    ..DEFAULT_SAMPLER
};

#[allow(missing_docs)]
pub const FRAME_TIME: Sampler = Sampler {
    label: "Frame time (ms)",
    expr: |s| s.frame_time,
    warn_threshold: 1000.0 / 55.0,
    err_threshold: 1000.0 / 25.0,
    precision: 2,
    ..DEFAULT_SAMPLER
};

#[allow(missing_docs)]
pub const WASM_MEMORY_USAGE: Sampler = Sampler {
    label: "WASM memory usage (Mb)",
    expr: |s| s.wasm_memory_usage as f64,
    warn_threshold: 50.0,
    err_threshold: 100.0,
    precision: 2,
    value_divisor: MB,
    ..DEFAULT_SAMPLER
};

#[allow(missing_docs)]
pub const GPU_MEMORY_USAGE: Sampler = Sampler {
    label: "GPU memory usage (Mb)",
    expr: |s| s.gpu_memory_usage as f64,
    warn_threshold: 100.0,
    err_threshold: 500.0,
    precision: 2,
    value_divisor: MB,
    ..DEFAULT_SAMPLER
};

#[allow(missing_docs)]
pub const DRAW_CALL_COUNT: Sampler = Sampler {
    label: "Draw call count",
    expr: |s| s.draw_calls.len() as f64,
    details: Some(|s| &s.draw_calls),
    warn_threshold: 100.0,
    err_threshold: 500.0,
    ..DEFAULT_SAMPLER
};

#[allow(missing_docs)]
pub const BUFFER_COUNT: Sampler = Sampler {
    label: "Buffer count",
    expr: |s| s.buffer_count as f64,
    warn_threshold: 100.0,
    err_threshold: 500.0,
    ..DEFAULT_SAMPLER
};

#[allow(missing_docs)]
pub const DATA_UPLOAD_COUNT: Sampler = Sampler {
    label: "Data upload count",
    expr: |s| s.data_upload_count as f64,
    warn_threshold: 100.0,
    err_threshold: 500.0,
    ..DEFAULT_SAMPLER
};

#[allow(missing_docs)]
pub const DATA_UPLOAD_SIZE: Sampler = Sampler {
    label: "Data upload size (Mb)",
    expr: |s| s.data_upload_size as f64,
    warn_threshold: 1.0,
    err_threshold: 10.0,
    precision: 2,
    value_divisor: MB,
    ..DEFAULT_SAMPLER
};

#[allow(missing_docs)]
pub const SPRITE_SYSTEM_COUNT: Sampler = Sampler {
    label: "Sprite system count",
    expr: |s| s.sprite_system_count as f64,
    warn_threshold: 100.0,
    err_threshold: 500.0,
    ..DEFAULT_SAMPLER
};

#[allow(missing_docs)]
pub const SYMBOL_COUNT: Sampler = Sampler {
    label: "Symbol count",
    expr: |s| s.symbol_count as f64,
    warn_threshold: 100.0,
    err_threshold: 500.0,
    ..DEFAULT_SAMPLER
};

#[allow(missing_docs)]
pub const SPRITE_COUNT: Sampler = Sampler {
    label: "Sprite count",
    expr: |s| s.sprite_count as f64,
    warn_threshold: 100_000.0,
    err_threshold: 500_000.0,
    ..DEFAULT_SAMPLER
};

#[allow(missing_docs)]
pub const SHADER_COUNT: Sampler = Sampler {
    label: "Shader count",
    expr: |s| s.shader_count as f64,
    warn_threshold: 100.0,
    err_threshold: 500.0,
    ..DEFAULT_SAMPLER
};

#[allow(missing_docs)]
pub const SHADER_COMPILE_COUNT: Sampler = Sampler {
    label: "Shader compile count",
    expr: |s| s.shader_compile_count as f64,
    warn_threshold: 10.0,
    err_threshold: 100.0,
    ..DEFAULT_SAMPLER
};
