const VRAM_START: u16 = 0x8000;
const VRAM_END: u16 = 0x9FFF;
const VRAM_SIZE: usize = ((VRAM_END - VRAM_START) + 1) as usize;
const OAM_START: u16 = 0xFE00;
const OAM_END: u16 = 0xFE9F;
const OAM_SIZE: usize = ((OAM_END - OAM_START) + 1) as usize;
const LCD_CONTROL_ADDRESS: u16 = 0xFF40;
const LCD_STATUS_ADDRESS: u16 = 0xFF41;
const BACKGROUND_Y_ADDRESS: u16 = 0xFF42;
const BACKGROUND_X_ADDRESS: u16 = 0xFF43;
const LCD_Y_ADDRESS: u16 = 0xFF44;
const LCD_Y_COMPARE_ADDRESS: u16 = 0xFF45;
const OAM_DMA_SOURCE_ADDRESS: u16 = 0xFF46;
const PALETTE_BG_ADDRESS: u16 = 0xFF47;
const PALETTE_OBJ_0_ADDRESS: u16 = 0xFF48;
const PALETTE_OBJ_1_ADDRESS: u16 = 0xFF49;
const WINDOW_Y_ADDRESS: u16 = 0xFF4A;
const WINDOW_X_ADDRESS: u16 = 0xFF4B;

pub struct Gpu {
    vram: [u8; VRAM_SIZE],
    oam: [u8; OAM_SIZE],
    lcd_control: LcdControl,
    lcd_status: LcdStatus,
    background_y: u8,
    background_x: u8,
    lcd_y: u8,
    lcd_y_compare: u8,
    oam_dma_source: u8,
    palette_bg: u8,
    palette_obj_0: u8,
    palette_obj_1: u8,
    window_y: u8,
    window_x: u8,
}

impl Gpu {
    pub fn new () -> Self {
        Self {
            vram: [0; VRAM_SIZE],
            oam: [0; OAM_SIZE],
            lcd_control: LcdControl::new(),
            lcd_status: LcdStatus::new(),
            background_y: 0,
            background_x: 0,
            lcd_y: 0,
            lcd_y_compare: 0,
            oam_dma_source: 0,
            palette_bg: 0,
            palette_obj_0: 0,
            palette_obj_1: 0,
            window_y: 0,
            window_x: 0,
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            VRAM_START..=VRAM_END => self.vram[(address - VRAM_START) as usize],
            OAM_START..=OAM_END => self.oam[(address - VRAM_START) as usize],
            LCD_CONTROL_ADDRESS => self.lcd_control.read_byte(),
            LCD_STATUS_ADDRESS => self.lcd_status.read_byte(),
            BACKGROUND_Y_ADDRESS => self.background_y,
            BACKGROUND_X_ADDRESS => self.background_x,
            LCD_Y_ADDRESS => self.lcd_y,
            LCD_Y_COMPARE_ADDRESS => self.lcd_y_compare,
            OAM_DMA_SOURCE_ADDRESS => self.oam_dma_source,
            WINDOW_Y_ADDRESS => self.window_y,
            WINDOW_X_ADDRESS => self.window_x, // TODO +- 7?
            _ => 0xFF
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            VRAM_START..=VRAM_END => self.vram[(address - VRAM_START) as usize] = value,
            OAM_START..=OAM_END => self.oam[(address - VRAM_START) as usize] = value,
            LCD_CONTROL_ADDRESS => self.lcd_control.write_byte(value),
            LCD_STATUS_ADDRESS => self.lcd_status.write_byte(value),
            BACKGROUND_Y_ADDRESS => self.background_y = value,
            BACKGROUND_X_ADDRESS => self.background_x = value,
            LCD_Y_ADDRESS => self.lcd_y = value,
            LCD_Y_COMPARE_ADDRESS => self.lcd_y_compare = value,
            OAM_DMA_SOURCE_ADDRESS => self.oam_dma_source = value,
            WINDOW_Y_ADDRESS => self.window_y = value,
            WINDOW_X_ADDRESS => self.window_x = value, // TODO +- 7?
            _ => {
                println!("Unhandled write to {} with {}", address, value);
            },
        }
    }

    // TODO probably need to add clock cycles
    pub fn step(&mut self) {

    }
}

#[derive(Copy, Clone)]
pub enum Mode {
    Hblank = 0,
    Vblank = 1,
    OamScan = 2,
    Draw = 3,
}

pub struct LcdStatus {
    // coincidence: LYC==LY
    coincidence_interrupt: bool,
    oam_scan_interrupt: bool,
    v_blank_interrupt: bool,
    h_blank_interrupt: bool,
    coincidence_flag: bool,
    mode: Mode,
}

impl LcdStatus {
    pub fn new() -> Self {
        Self {
            coincidence_interrupt: false,
            oam_scan_interrupt: false,
            v_blank_interrupt: false,
            h_blank_interrupt: false,
            coincidence_flag: false,
            mode: Mode::Hblank,
        }
    }

    pub fn read_byte(&self) -> u8 {
        let mut ret: u8 = 0;
        if self.coincidence_interrupt { ret = ret | 0b0100_0000 }
        if self.oam_scan_interrupt    { ret = ret | 0b0010_0000 }
        if self.v_blank_interrupt     { ret = ret | 0b0001_0000 }
        if self.h_blank_interrupt     { ret = ret | 0b0000_1000 }
        if self.coincidence_flag      { ret = ret | 0b0000_0100 }

        ret | (self.mode as u8)
    }

    pub fn write_byte(&mut self, data: u8) {
        self.coincidence_interrupt = data & 0b0100_0000 > 0;
        self.oam_scan_interrupt    = data & 0b0010_0000 > 0;
        self.v_blank_interrupt     = data & 0b0001_0000 > 0;
        self.h_blank_interrupt     = data & 0b0000_1000 > 0;
        self.coincidence_flag      = data & 0b0000_0100 > 0;

        let mode_bits = data & 0b0000_0011;
        match mode_bits {
            0 => self.mode = Mode::Hblank,
            1 => self.mode = Mode::Vblank,
            2 => self.mode = Mode::OamScan,
            3 => self.mode = Mode::Draw,
            _ => panic!("Unexpected LcdStatus Mode")
        }
    }
}

pub struct LcdControl {
    enabled: bool,
    window_tile_map: bool,
    window_enabled: bool,
    tile_data_select: bool,
    background_tile_map_select: bool,
    large_sprite_size_enabled: bool,
    sprites_enabled: bool,
    background_and_window_enabled: bool,
}

impl LcdControl {
    pub fn new() -> Self {
        Self {
            enabled: false,
            window_tile_map: false,
            window_enabled: false,
            tile_data_select: false,
            background_tile_map_select: false,
            large_sprite_size_enabled: false,
            sprites_enabled: false,
            background_and_window_enabled: false,
        }
    }

    pub fn read_byte(&self) -> u8 {
        let mut control_byte: u8 = 0;

        if self.enabled                       { control_byte |= 0b1000_0000 }
        if self.window_tile_map               { control_byte |= 0b0100_0000 }
        if self.window_enabled                { control_byte |= 0b0010_0000 }
        if self.tile_data_select              { control_byte |= 0b0001_0000 }
        if self.background_tile_map_select    { control_byte |= 0b0000_1000 }
        if self.large_sprite_size_enabled     { control_byte |= 0b0000_0100 }
        if self.sprites_enabled               { control_byte |= 0b0000_0010 }
        if self.background_and_window_enabled { control_byte |= 0b0000_0001 }

        control_byte
    }

    pub fn write_byte(&mut self, data: u8) {
        self.enabled                       = data & 0b1000_0000 > 0;
        self.window_tile_map               = data & 0b0100_0000 > 0;
        self.window_enabled                = data & 0b0010_0000 > 0;
        self.tile_data_select              = data & 0b0001_0000 > 0;
        self.background_tile_map_select    = data & 0b0000_1000 > 0;
        self.large_sprite_size_enabled     = data & 0b0000_0100 > 0;
        self.sprites_enabled               = data & 0b0000_0010 > 0;
        self.background_and_window_enabled = data & 0b0000_0001 > 0;
    }
}