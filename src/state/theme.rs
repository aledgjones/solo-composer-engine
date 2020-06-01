use crate::state::Engine;
use crate::utils::storage::Storage;
use serde_repr::*;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
pub struct Shade {
    background: String,
    foreground: String,
}

#[derive(Serialize)]
pub struct Pallet {
    shade_200: Shade,
    shade_300: Shade,
    shade_400: Shade,
    shade_500: Shade,
    shade_600: Shade,
    shade_700: Shade,
    shade_800: Shade,
}

#[wasm_bindgen]
#[derive(Serialize_repr)]
#[repr(u8)]
pub enum ThemeMode {
    Light,
    Dark,
}

impl ThemeMode {
    pub fn to_pallet(&self) -> Pallet {
        match self {
            ThemeMode::Light => Pallet {
                shade_200: Shade {
                    background: String::from("#ffffff"),
                    foreground: String::from("#323232"),
                },
                shade_300: Shade {
                    background: String::from("#e9e9e9"),
                    foreground: String::from("#323232"),
                },
                shade_400: Shade {
                    background: String::from("#e9e9e9"),
                    foreground: String::from("#323232"),
                },
                shade_500: Shade {
                    background: String::from("#e0e0e0"),
                    foreground: String::from("#323232"),
                },
                shade_600: Shade {
                    background: String::from("#d7d7d7"),
                    foreground: String::from("#323232"),
                },
                shade_700: Shade {
                    background: String::from("#d1d1d1"),
                    foreground: String::from("#323232"),
                },
                shade_800: Shade {
                    background: String::from("#afafaf"),
                    foreground: String::from("#323232"),
                },
            },
            ThemeMode::Dark => Pallet {
                shade_200: Shade {
                    background: String::from("#101010"),
                    foreground: String::from("#ffffff"),
                },
                shade_300: Shade {
                    background: String::from("#131313"),
                    foreground: String::from("#ffffff"),
                },
                shade_400: Shade {
                    background: String::from("#161616"),
                    foreground: String::from("#ffffff"),
                },
                shade_500: Shade {
                    background: String::from("#1f1f1f"),
                    foreground: String::from("#ffffff"),
                },
                shade_600: Shade {
                    background: String::from("#282828"),
                    foreground: String::from("#ffffff"),
                },
                shade_700: Shade {
                    background: String::from("#2E2E2E"),
                    foreground: String::from("#ffffff"),
                },
                shade_800: Shade {
                    background: String::from("#505050"),
                    foreground: String::from("#ffffff"),
                },
            },
        }
    }
    /**
     * Convert the enum to a string that can be easily stored in localStorage
     */
    pub fn to_string(&self) -> String {
        match self {
            ThemeMode::Light => String::from("light"),
            ThemeMode::Dark => String::from("dark"),
        }
    }

    /**
     * Convert a string to a ThemeMode used when retreiving the value from localStorage
     */
    pub fn from_string(mode: String) -> ThemeMode {
        if mode == "light" {
            ThemeMode::Light
        } else {
            ThemeMode::Dark
        }
    }
}

#[derive(Serialize)]
pub struct Pallets {
    background: Pallet,
    primary: Pallet,
    highlight: Pallet,
    error: Pallet,
}

#[derive(Serialize)]
pub struct Theme {
    mode: ThemeMode,
    pallets: Pallets,
}

impl Theme {
    pub fn new() -> Theme {
        let mode = if let Ok(store) = Storage::new() {
            let value = store.get("sc/theme-mode");
            match value {
                Some(value) => ThemeMode::from_string(value),
                None => ThemeMode::Dark,
            }
        } else {
            ThemeMode::Dark
        };

        Theme {
            pallets: Pallets {
                background: mode.to_pallet(),
                primary: Pallet {
                    shade_200: Shade {
                        background: String::from("#00508e"),
                        foreground: String::from("#ffffff"),
                    },
                    shade_300: Shade {
                        background: String::from("#00508e"),
                        foreground: String::from("#ffffff"),
                    },
                    shade_400: Shade {
                        background: String::from("#00508e"),
                        foreground: String::from("#ffffff"),
                    },
                    shade_500: Shade {
                        background: String::from("#0064b1"),
                        foreground: String::from("#ffffff"),
                    },
                    shade_600: Shade {
                        background: String::from("#1a74b9"),
                        foreground: String::from("#ffffff"),
                    },
                    shade_700: Shade {
                        background: String::from("#3383c1"),
                        foreground: String::from("#ffffff"),
                    },
                    shade_800: Shade {
                        background: String::from("#4d93c9"),
                        foreground: String::from("#ffffff"),
                    },
                },
                highlight: Pallet {
                    shade_200: Shade {
                        background: String::from("#ffa500"),
                        foreground: String::from("#ffffff"),
                    },
                    shade_300: Shade {
                        background: String::from("#ffa500"),
                        foreground: String::from("#ffffff"),
                    },
                    shade_400: Shade {
                        background: String::from("#ffa500"),
                        foreground: String::from("#ffffff"),
                    },
                    shade_500: Shade {
                        background: String::from("#ffa500"),
                        foreground: String::from("#ffffff"),
                    },
                    shade_600: Shade {
                        background: String::from("#ffa500"),
                        foreground: String::from("#ffffff"),
                    },
                    shade_700: Shade {
                        background: String::from("#ffa500"),
                        foreground: String::from("#ffffff"),
                    },
                    shade_800: Shade {
                        background: String::from("#ffa500"),
                        foreground: String::from("#ffffff"),
                    },
                },
                error: Pallet {
                    shade_200: Shade {
                        background: String::from("#ff6347"),
                        foreground: String::from("#ffffff"),
                    },
                    shade_300: Shade {
                        background: String::from("#ff6347"),
                        foreground: String::from("#ffffff"),
                    },
                    shade_400: Shade {
                        background: String::from("#ff6347"),
                        foreground: String::from("#ffffff"),
                    },
                    shade_500: Shade {
                        background: String::from("#ff6347"),
                        foreground: String::from("#ffffff"),
                    },
                    shade_600: Shade {
                        background: String::from("#ff6347"),
                        foreground: String::from("#ffffff"),
                    },
                    shade_700: Shade {
                        background: String::from("#ff6347"),
                        foreground: String::from("#ffffff"),
                    },
                    shade_800: Shade {
                        background: String::from("#ff6347"),
                        foreground: String::from("#ffffff"),
                    },
                },
            },
            mode,
        }
    }
}

#[wasm_bindgen]
impl Engine {
    pub fn set_theme(&mut self, value: ThemeMode) {
        let string = value.to_string();
        self.state.app.theme.pallets.background = value.to_pallet();
        self.state.app.theme.mode = value;
        self.emit();
        if let Ok(store) = Storage::new() {
            store.set("sc/theme-mode", &string);
        }
    }
}
