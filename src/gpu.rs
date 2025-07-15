const VRAM_START: u16 = 0x8000;
const VRAM_END: u16 = 0x9FFF;
const VRAM_SIZE: usize = ((VRAM_END - VRAM_START) + 1) as usize;
const OAM_START: u16 = 0xFE00;
const OAM_END: u16 = 0xFE9F;
const OAM_SIZE: usize = ((OAM_END - OAM_START) + 1) as usize;
const LCD_CONTROL_ADDRESS: u16 = 0xFF40;
const LCD_STAT_ADDRESS: u16 = 0xFF41;
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
    lcd_control: u8,
    lcd_stat: u8,
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
            lcd_control: 0,
            lcd_stat: 0,
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
            LCD_CONTROL_ADDRESS => self.lcd_control,
            LCD_STAT_ADDRESS => self.lcd_stat,
            BACKGROUND_Y_ADDRESS => self.background_y,
            BACKGROUND_X_ADDRESS => self.background_x,
            LCD_Y_ADDRESS => self.lcd_y,
            LCD_Y_COMPARE_ADDRESS => self.lcd_y_compare,
            OAM_DMA_SOURCE_ADDRESS => self.oam_dma_source,
            WINDOW_Y_ADDRESS => self.window_y,
            WINDOW_X_ADDRESS => self.window_x + 7, // weird that its + 7
            _ => 0xFF
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            VRAM_START..=VRAM_END => self.vram[(address - VRAM_START) as usize] = value,
            OAM_START..=OAM_END => self.oam[(address - VRAM_START) as usize] = value,
            LCD_CONTROL_ADDRESS => self.lcd_control = value,
            LCD_STAT_ADDRESS => self.lcd_stat = value,
            BACKGROUND_Y_ADDRESS => self.background_y = value,
            BACKGROUND_X_ADDRESS => self.background_x = value,
            LCD_Y_ADDRESS => self.lcd_y = value,
            LCD_Y_COMPARE_ADDRESS => self.lcd_y_compare = value,
            OAM_DMA_SOURCE_ADDRESS => self.oam_dma_source = value,
            WINDOW_Y_ADDRESS => self.window_y = value,
            WINDOW_X_ADDRESS => self.window_x = value.saturating_sub(7), // weird that its - 7
            _ => {
                println!("Unhandled write to {} with {}", address, value);
            },
        }
    }

    pub fn read_lcd_control_flag(&self, lcd_control_flag: LcdControlFlag) -> bool {
        self.lcd_control & (lcd_control_flag as u8) > 0
    }

    pub fn write_lcd_control_flag(&mut self, lcd_control_flag: LcdControlFlag, value: bool) {
        if value {
            let mask = lcd_control_flag as u8;
            self.lcd_control = self.lcd_control | mask;
        } else {
            let mask = !(lcd_control_flag as u8);
            self.lcd_control = self.lcd_control & mask;
        }
    }
}

pub enum LcdControlFlag {
    Enable = 0b1000_0000,
    WindowTileMap = 0b0100_0000,
    WindowEnable = 0b0010_0000,
    BgAndWindowTileAreaData = 0b0001_0000,
    BgTileMapArea = 0b0000_1000,
    ObjectSize = 0b0000_0100,
    ObjectEnable = 0b0000_0010,
    BgAndWindowEnable = 0b0000_0001,
}