use crate::gradient::EvalGradient;
use crate::{interpolate, Color, Gradient};

pub const TURBO: Gradient = Gradient { eval: &Turbo };

struct Turbo;

impl EvalGradient for Turbo {
    fn name(&self) -> &'static str {
        "Turbo"
    }

    fn eval_continuous(&self, t: f64) -> Color {
        let r = (34.61
            + t * (1172.33 - t * (10793.56 - t * (33300.12 - t * (38394.49 - t * 14825.05)))))
            .max(0.0)
            .min(255.0) as u8;
        let g = (23.31 + t * (557.33 + t * (1225.33 - t * (3574.96 - t * (1073.77 + t * 707.56)))))
            .max(0.0)
            .min(255.0) as u8;
        let b = (27.2
            + t * (3211.1 - t * (15327.97 - t * (27814.0 - t * (22569.18 - t * 6838.66)))))
            .max(0.0)
            .min(255.0) as u8;
        Color { r, g, b }
    }
}

struct Ramp {
    name: &'static str,
    colors: &'static [Color],
}

impl EvalGradient for Ramp {
    fn name(&self) -> &'static str {
        self.name
    }

    fn eval_continuous(&self, t: f64) -> Color {
        interpolate::spline(&self.colors, t)
    }
}

pub const VIRIDIS: Gradient = Gradient {
    eval: &Ramp {
        name: "Viridis",
        colors: &colors! {
            0x440154 0x440256 0x450457 0x450559 0x46075a 0x46085c 0x460a5d 0x460b5e
            0x470d60 0x470e61 0x471063 0x471164 0x471365 0x481467 0x481668 0x481769
            0x48186a 0x481a6c 0x481b6d 0x481c6e 0x481d6f 0x481f70 0x482071 0x482173
            0x482374 0x482475 0x482576 0x482677 0x482878 0x482979 0x472a7a 0x472c7a
            0x472d7b 0x472e7c 0x472f7d 0x46307e 0x46327e 0x46337f 0x463480 0x453581
            0x453781 0x453882 0x443983 0x443a83 0x443b84 0x433d84 0x433e85 0x423f85
            0x424086 0x424186 0x414287 0x414487 0x404588 0x404688 0x3f4788 0x3f4889
            0x3e4989 0x3e4a89 0x3e4c8a 0x3d4d8a 0x3d4e8a 0x3c4f8a 0x3c508b 0x3b518b
            0x3b528b 0x3a538b 0x3a548c 0x39558c 0x39568c 0x38588c 0x38598c 0x375a8c
            0x375b8d 0x365c8d 0x365d8d 0x355e8d 0x355f8d 0x34608d 0x34618d 0x33628d
            0x33638d 0x32648e 0x32658e 0x31668e 0x31678e 0x31688e 0x30698e 0x306a8e
            0x2f6b8e 0x2f6c8e 0x2e6d8e 0x2e6e8e 0x2e6f8e 0x2d708e 0x2d718e 0x2c718e
            0x2c728e 0x2c738e 0x2b748e 0x2b758e 0x2a768e 0x2a778e 0x2a788e 0x29798e
            0x297a8e 0x297b8e 0x287c8e 0x287d8e 0x277e8e 0x277f8e 0x27808e 0x26818e
            0x26828e 0x26828e 0x25838e 0x25848e 0x25858e 0x24868e 0x24878e 0x23888e
            0x23898e 0x238a8d 0x228b8d 0x228c8d 0x228d8d 0x218e8d 0x218f8d 0x21908d
            0x21918c 0x20928c 0x20928c 0x20938c 0x1f948c 0x1f958b 0x1f968b 0x1f978b
            0x1f988b 0x1f998a 0x1f9a8a 0x1e9b8a 0x1e9c89 0x1e9d89 0x1f9e89 0x1f9f88
            0x1fa088 0x1fa188 0x1fa187 0x1fa287 0x20a386 0x20a486 0x21a585 0x21a685
            0x22a785 0x22a884 0x23a983 0x24aa83 0x25ab82 0x25ac82 0x26ad81 0x27ad81
            0x28ae80 0x29af7f 0x2ab07f 0x2cb17e 0x2db27d 0x2eb37c 0x2fb47c 0x31b57b
            0x32b67a 0x34b679 0x35b779 0x37b878 0x38b977 0x3aba76 0x3bbb75 0x3dbc74
            0x3fbc73 0x40bd72 0x42be71 0x44bf70 0x46c06f 0x48c16e 0x4ac16d 0x4cc26c
            0x4ec36b 0x50c46a 0x52c569 0x54c568 0x56c667 0x58c765 0x5ac864 0x5cc863
            0x5ec962 0x60ca60 0x63cb5f 0x65cb5e 0x67cc5c 0x69cd5b 0x6ccd5a 0x6ece58
            0x70cf57 0x73d056 0x75d054 0x77d153 0x7ad151 0x7cd250 0x7fd34e 0x81d34d
            0x84d44b 0x86d549 0x89d548 0x8bd646 0x8ed645 0x90d743 0x93d741 0x95d840
            0x98d83e 0x9bd93c 0x9dd93b 0xa0da39 0xa2da37 0xa5db36 0xa8db34 0xaadc32
            0xaddc30 0xb0dd2f 0xb2dd2d 0xb5de2b 0xb8de29 0xbade28 0xbddf26 0xc0df25
            0xc2df23 0xc5e021 0xc8e020 0xcae11f 0xcde11d 0xd0e11c 0xd2e21b 0xd5e21a
            0xd8e219 0xdae319 0xdde318 0xdfe318 0xe2e418 0xe5e419 0xe7e419 0xeae51a
            0xece51b 0xefe51c 0xf1e51d 0xf4e61e 0xf6e620 0xf8e621 0xfbe723 0xfde725
        },
    },
};

#[derive(Copy, Clone)]
struct SequentialMulti {
    name: &'static str,
    three: [Color; 3],
    four: [Color; 4],
    five: [Color; 5],
    six: [Color; 6],
    seven: [Color; 7],
    eight: [Color; 8],
    nine: [Color; 9],
}

impl EvalGradient for SequentialMulti {
    fn name(&self) -> &'static str {
        self.name
    }

    fn eval_rational(&self, i: usize, n: usize) -> Color {
        match n {
            0 | 1 => self.three[2],
            2 => self.three[i * 2],
            3 => self.three[i],
            4 => self.four[i],
            5 => self.five[i],
            6 => self.six[i],
            7 => self.seven[i],
            8 => self.eight[i],
            9 => self.nine[i],
            _ => self.eval_continuous(i as f64 / (n - 1) as f64),
        }
    }

    fn eval_continuous(&self, t: f64) -> Color {
        interpolate::spline(&self.nine, t)
    }
}

pub const BLUE_GREEN: Gradient = Gradient {
    eval: &SequentialMulti {
        name: "BuGn",
        three: colors!(0xe5f5f9 0x99d8c9 0x2ca25f),
        four: colors!(0xedf8fb 0xb2e2e2 0x66c2a4 0x238b45),
        five: colors!(0xedf8fb 0xb2e2e2 0x66c2a4 0x2ca25f 0x006d2c),
        six: colors!(0xedf8fb 0xccece6 0x99d8c9 0x66c2a4 0x2ca25f 0x006d2c),
        seven: colors!(0xedf8fb 0xccece6 0x99d8c9 0x66c2a4 0x41ae76 0x238b45 0x005824),
        eight: colors!(0xf7fcfd 0xe5f5f9 0xccece6 0x99d8c9 0x66c2a4 0x41ae76 0x238b45 0x005824),
        nine: colors!(0xf7fcfd 0xe5f5f9 0xccece6 0x99d8c9 0x66c2a4 0x41ae76 0x238b45 0x006d2c 0x00441b),
    },
};

pub const BLUE_PURPLE: Gradient = Gradient {
    eval: &SequentialMulti {
        name: "BuPu",
        three: colors!(0xe0ecf4 0x9ebcda 0x8856a7),
        four: colors!(0xedf8fb 0xb3cde3 0x8c96c6 0x88419d),
        five: colors!(0xedf8fb 0xb3cde3 0x8c96c6 0x8856a7 0x810f7c),
        six: colors!(0xedf8fb 0xbfd3e6 0x9ebcda 0x8c96c6 0x8856a7 0x810f7c),
        seven: colors!(0xedf8fb 0xbfd3e6 0x9ebcda 0x8c96c6 0x8c6bb1 0x88419d 0x6e016b),
        eight: colors!(0xf7fcfd 0xe0ecf4 0xbfd3e6 0x9ebcda 0x8c96c6 0x8c6bb1 0x88419d 0x6e016b),
        nine: colors!(0xf7fcfd 0xe0ecf4 0xbfd3e6 0x9ebcda 0x8c96c6 0x8c6bb1 0x88419d 0x810f7c 0x4d004b),
    },
};

pub const GREEN_BLUE: Gradient = Gradient {
    eval: &SequentialMulti {
        name: "GnBu",
        three: colors!(0xe0f3db 0xa8ddb5 0x43a2ca),
        four: colors!(0xf0f9e8 0xbae4bc 0x7bccc4 0x2b8cbe),
        five: colors!(0xf0f9e8 0xbae4bc 0x7bccc4 0x43a2ca 0x0868ac),
        six: colors!(0xf0f9e8 0xccebc5 0xa8ddb5 0x7bccc4 0x43a2ca 0x0868ac),
        seven: colors!(0xf0f9e8 0xccebc5 0xa8ddb5 0x7bccc4 0x4eb3d3 0x2b8cbe 0x08589e),
        eight: colors!(0xf7fcf0 0xe0f3db 0xccebc5 0xa8ddb5 0x7bccc4 0x4eb3d3 0x2b8cbe 0x08589e),
        nine: colors!(0xf7fcf0 0xe0f3db 0xccebc5 0xa8ddb5 0x7bccc4 0x4eb3d3 0x2b8cbe 0x0868ac 0x084081),
    },
};

pub const ORANGE_RED: Gradient = Gradient {
    eval: &SequentialMulti {
        name: "OrRd",
        three: colors!(0xfee8c8 0xfdbb84 0xe34a33),
        four: colors!(0xfef0d9 0xfdcc8a 0xfc8d59 0xd7301f),
        five: colors!(0xfef0d9 0xfdcc8a 0xfc8d59 0xe34a33 0xb30000),
        six: colors!(0xfef0d9 0xfdd49e 0xfdbb84 0xfc8d59 0xe34a33 0xb30000),
        seven: colors!(0xfef0d9 0xfdd49e 0xfdbb84 0xfc8d59 0xef6548 0xd7301f 0x990000),
        eight: colors!(0xfff7ec 0xfee8c8 0xfdd49e 0xfdbb84 0xfc8d59 0xef6548 0xd7301f 0x990000),
        nine: colors!(0xfff7ec 0xfee8c8 0xfdd49e 0xfdbb84 0xfc8d59 0xef6548 0xd7301f 0xb30000 0x7f0000),
    },
};

pub const PURPLE_BLUE_GREEN: Gradient = Gradient {
    eval: &SequentialMulti {
        name: "PuBuGn",
        three: colors!(0xece2f0 0xa6bddb 0x1c9099),
        four: colors!(0xf6eff7 0xbdc9e1 0x67a9cf 0x02818a),
        five: colors!(0xf6eff7 0xbdc9e1 0x67a9cf 0x1c9099 0x016c59),
        six: colors!(0xf6eff7 0xd0d1e6 0xa6bddb 0x67a9cf 0x1c9099 0x016c59),
        seven: colors!(0xf6eff7 0xd0d1e6 0xa6bddb 0x67a9cf 0x3690c0 0x02818a 0x016450),
        eight: colors!(0xfff7fb 0xece2f0 0xd0d1e6 0xa6bddb 0x67a9cf 0x3690c0 0x02818a 0x016450),
        nine: colors!(0xfff7fb 0xece2f0 0xd0d1e6 0xa6bddb 0x67a9cf 0x3690c0 0x02818a 0x016c59 0x014636),
    },
};

pub const PURPLE_BLUE: Gradient = Gradient {
    eval: &SequentialMulti {
        name: "PuBu",
        three: colors!(0xece7f2 0xa6bddb 0x2b8cbe),
        four: colors!(0xf1eef6 0xbdc9e1 0x74a9cf 0x0570b0),
        five: colors!(0xf1eef6 0xbdc9e1 0x74a9cf 0x2b8cbe 0x045a8d),
        six: colors!(0xf1eef6 0xd0d1e6 0xa6bddb 0x74a9cf 0x2b8cbe 0x045a8d),
        seven: colors!(0xf1eef6 0xd0d1e6 0xa6bddb 0x74a9cf 0x3690c0 0x0570b0 0x034e7b),
        eight: colors!(0xfff7fb 0xece7f2 0xd0d1e6 0xa6bddb 0x74a9cf 0x3690c0 0x0570b0 0x034e7b),
        nine: colors!(0xfff7fb 0xece7f2 0xd0d1e6 0xa6bddb 0x74a9cf 0x3690c0 0x0570b0 0x045a8d 0x023858),
    },
};

pub const PURPLE_RED: Gradient = Gradient {
    eval: &SequentialMulti {
        name: "PuRd",
        three: colors!(0xe7e1ef 0xc994c7 0xdd1c77),
        four: colors!(0xf1eef6 0xd7b5d8 0xdf65b0 0xce1256),
        five: colors!(0xf1eef6 0xd7b5d8 0xdf65b0 0xdd1c77 0x980043),
        six: colors!(0xf1eef6 0xd4b9da 0xc994c7 0xdf65b0 0xdd1c77 0x980043),
        seven: colors!(0xf1eef6 0xd4b9da 0xc994c7 0xdf65b0 0xe7298a 0xce1256 0x91003f),
        eight: colors!(0xf7f4f9 0xe7e1ef 0xd4b9da 0xc994c7 0xdf65b0 0xe7298a 0xce1256 0x91003f),
        nine: colors!(0xf7f4f9 0xe7e1ef 0xd4b9da 0xc994c7 0xdf65b0 0xe7298a 0xce1256 0x980043 0x67001f),
    },
};

pub const RED_PURPLE: Gradient = Gradient {
    eval: &SequentialMulti {
        name: "RdPu",
        three: colors!(0xfde0dd 0xfa9fb5 0xc51b8a),
        four: colors!(0xfeebe2 0xfbb4b9 0xf768a1 0xae017e),
        five: colors!(0xfeebe2 0xfbb4b9 0xf768a1 0xc51b8a 0x7a0177),
        six: colors!(0xfeebe2 0xfcc5c0 0xfa9fb5 0xf768a1 0xc51b8a 0x7a0177),
        seven: colors!(0xfeebe2 0xfcc5c0 0xfa9fb5 0xf768a1 0xdd3497 0xae017e 0x7a0177),
        eight: colors!(0xfff7f3 0xfde0dd 0xfcc5c0 0xfa9fb5 0xf768a1 0xdd3497 0xae017e 0x7a0177),
        nine: colors!(0xfff7f3 0xfde0dd 0xfcc5c0 0xfa9fb5 0xf768a1 0xdd3497 0xae017e 0x7a0177 0x49006a),
    },
};

pub const YELLOW_GREEN_BLUE: Gradient = Gradient {
    eval: &SequentialMulti {
        name: "YlGnBu",
        three: colors!(0xedf8b1 0x7fcdbb 0x2c7fb8),
        four: colors!(0xffffcc 0xa1dab4 0x41b6c4 0x225ea8),
        five: colors!(0xffffcc 0xa1dab4 0x41b6c4 0x2c7fb8 0x253494),
        six: colors!(0xffffcc 0xc7e9b4 0x7fcdbb 0x41b6c4 0x2c7fb8 0x253494),
        seven: colors!(0xffffcc 0xc7e9b4 0x7fcdbb 0x41b6c4 0x1d91c0 0x225ea8 0x0c2c84),
        eight: colors!(0xffffd9 0xedf8b1 0xc7e9b4 0x7fcdbb 0x41b6c4 0x1d91c0 0x225ea8 0x0c2c84),
        nine: colors!(0xffffd9 0xedf8b1 0xc7e9b4 0x7fcdbb 0x41b6c4 0x1d91c0 0x225ea8 0x253494 0x081d58),
    },
};

pub const YELLOW_GREEN: Gradient = Gradient {
    eval: &SequentialMulti {
        name: "YlGn",
        three: colors!(0xf7fcb9 0xaddd8e 0x31a354),
        four: colors!(0xffffcc 0xc2e699 0x78c679 0x238443),
        five: colors!(0xffffcc 0xc2e699 0x78c679 0x31a354 0x006837),
        six: colors!(0xffffcc 0xd9f0a3 0xaddd8e 0x78c679 0x31a354 0x006837),
        seven: colors!(0xffffcc 0xd9f0a3 0xaddd8e 0x78c679 0x41ab5d 0x238443 0x005a32),
        eight: colors!(0xffffe5 0xf7fcb9 0xd9f0a3 0xaddd8e 0x78c679 0x41ab5d 0x238443 0x005a32),
        nine: colors!(0xffffe5 0xf7fcb9 0xd9f0a3 0xaddd8e 0x78c679 0x41ab5d 0x238443 0x006837 0x004529),
    },
};

pub const YELLOW_ORANGE_BROWN: Gradient = Gradient {
    eval: &SequentialMulti {
        name: "YlOrBr",
        three: colors!(0xfff7bc 0xfec44f 0xd95f0e),
        four: colors!(0xffffd4 0xfed98e 0xfe9929 0xcc4c02),
        five: colors!(0xffffd4 0xfed98e 0xfe9929 0xd95f0e 0x993404),
        six: colors!(0xffffd4 0xfee391 0xfec44f 0xfe9929 0xd95f0e 0x993404),
        seven: colors!(0xffffd4 0xfee391 0xfec44f 0xfe9929 0xec7014 0xcc4c02 0x8c2d04),
        eight: colors!(0xffffe5 0xfff7bc 0xfee391 0xfec44f 0xfe9929 0xec7014 0xcc4c02 0x8c2d04),
        nine: colors!(0xffffe5 0xfff7bc 0xfee391 0xfec44f 0xfe9929 0xec7014 0xcc4c02 0x993404 0x662506),
    },
};

pub const YELLOW_ORANGE_RED: Gradient = Gradient {
    eval: &SequentialMulti {
        name: "YlOrRd",
        three: colors!(0xffeda0 0xfeb24c 0xf03b20),
        four: colors!(0xffffb2 0xfecc5c 0xfd8d3c 0xe31a1c),
        five: colors!(0xffffb2 0xfecc5c 0xfd8d3c 0xf03b20 0xbd0026),
        six: colors!(0xffffb2 0xfed976 0xfeb24c 0xfd8d3c 0xf03b20 0xbd0026),
        seven: colors!(0xffffb2 0xfed976 0xfeb24c 0xfd8d3c 0xfc4e2a 0xe31a1c 0xb10026),
        eight: colors!(0xffffcc 0xffeda0 0xfed976 0xfeb24c 0xfd8d3c 0xfc4e2a 0xe31a1c 0xb10026),
        nine: colors!(0xffffcc 0xffeda0 0xfed976 0xfeb24c 0xfd8d3c 0xfc4e2a 0xe31a1c 0xbd0026 0x800026),
    },
};
