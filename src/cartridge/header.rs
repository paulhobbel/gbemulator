static OLD_LICENSE_CODE: [Option<&'static str>; 0x100] = {
    let mut codes = [None; 0x100];
    codes[0x00] = Some("None");
    codes[0x01] = Some("Nintendo");
    codes[0x08] = Some("Capcom");
    codes[0x09] = Some("HOT-B");
    codes[0x0A] = Some("Jaleco");
    codes[0x0B] = Some("Coconuts Japan");
    codes[0x0C] = Some("Elite Systems");
    codes[0x13] = Some("EA (Electronic Arts)");
    codes[0x18] = Some("Hudson Soft");
    codes[0x19] = Some("ITC Entertainment");
    codes[0x1A] = Some("Yanoman");
    codes[0x1D] = Some("Japan Clary");
    codes[0x1F] = Some("Virgin Games Ltd.");
    codes[0x24] = Some("PCM Complete");
    codes[0x25] = Some("San-X");
    codes[0x28] = Some("Kemco");
    codes[0x29] = Some("SETA Corporation");
    codes[0x30] = Some("Infogrames");
    codes[0x31] = Some("Nintendo");
    codes[0x32] = Some("Bandai");
    codes[0x34] = Some("Konami");
    codes[0x35] = Some("HectorSoft");
    codes[0x38] = Some("Capcom");
    codes[0x39] = Some("Banpresto");
    codes[0x3C] = Some("Entertainment Interactive");
    codes[0x3E] = Some("Gremlin");
    codes[0x41] = Some("Ubi Soft");
    codes[0x42] = Some("Atlus");
    codes[0x44] = Some("Malibu Interactive");
    codes[0x46] = Some("Angel");
    codes[0x47] = Some("Spectrum HoloByte");
    codes[0x49] = Some("Irem");
    codes[0x4A] = Some("Virgin Games Ltd.");
    codes[0x4D] = Some("Malibu Interactive");
    codes[0x4F] = Some("U.S. Gold");
    codes[0x50] = Some("Absolute");
    codes[0x51] = Some("Acclaim Entertainment");
    codes[0x52] = Some("Activision");
    codes[0x53] = Some("Sammy USA Corporation");
    codes[0x54] = Some("GameTek");
    codes[0x55] = Some("Park Place");
    codes[0x56] = Some("LJN");
    codes[0x57] = Some("Matchbox");
    codes[0x59] = Some("Milton Bradley Company");
    codes[0x5A] = Some("Mindscape");
    codes[0x5B] = Some("Romstar");
    codes[0x5C] = Some("Naxat Soft");
    codes[0x5D] = Some("Tradewest");
    codes[0x60] = Some("Titus Interactive");
    codes[0x61] = Some("Virgin Games Ltd.");
    codes[0x67] = Some("Ocean Software");
    codes[0x69] = Some("EA (Electronic Arts)");
    codes[0x6E] = Some("Elite Systems");
    codes[0x6F] = Some("Electro Brain");
    codes[0x70] = Some("Infogrames");
    codes[0x71] = Some("Interplay Entertainment");
    codes[0x72] = Some("Broderbund");
    codes[0x73] = Some("Sculptured Software");
    codes[0x75] = Some("The Sales Curve Limited");
    codes[0x78] = Some("THQ");
    codes[0x79] = Some("Accolade");
    codes[0x7A] = Some("Triffix Entertainment");
    codes[0x7C] = Some("MicroProse");
    codes[0x7F] = Some("Kemco");
    codes[0x80] = Some("Misawa Entertainment");
    codes[0x83] = Some("LOZC G.");
    codes[0x86] = Some("Tokuma Shoten");
    codes[0x8B] = Some("Bullet-Proof Software");
    codes[0x8C] = Some("Vic Tokai Corp.");
    codes[0x8E] = Some("Ape Inc.");
    codes[0x8F] = Some("I'Max");
    codes[0x91] = Some("Chunsoft Co.");
    codes[0x92] = Some("Video System");
    codes[0x93] = Some("Tsubaraya Productions");
    codes[0x95] = Some("Varie");
    codes[0x96] = Some("Yonezawa/S'Pal");
    codes[0x97] = Some("Kemco");
    codes[0x99] = Some("Arc");
    codes[0x9A] = Some("Nihon Bussan");
    codes[0x9B] = Some("Tecmo");
    codes[0x9C] = Some("Imagineer");
    codes[0x9D] = Some("Banpresto");
    codes[0x9F] = Some("Nova");
    codes[0xA1] = Some("Hori Electric");
    codes[0xA2] = Some("Bandai");
    codes[0xA4] = Some("Konami");
    codes[0xA6] = Some("Kawada");
    codes[0xA7] = Some("Takara");
    codes[0xA9] = Some("Technos Japan");
    codes[0xAA] = Some("Broderbund");
    codes[0xAC] = Some("Toei Animation");
    codes[0xAD] = Some("Toho");
    codes[0xAF] = Some("Namco");
    codes[0xB0] = Some("Acclaim Entertainment");
    codes[0xB1] = Some("ASCII Corporation or Nexsoft");
    codes[0xB2] = Some("Bandai");
    codes[0xB4] = Some("Square Enix");
    codes[0xB6] = Some("HAL Laboratory");
    codes[0xB7] = Some("SNK");
    codes[0xB9] = Some("Pony Canyon");
    codes[0xBA] = Some("Culture Brain");
    codes[0xBB] = Some("Sunsoft");
    codes[0xBD] = Some("Sony Imagesoft");
    codes[0xBF] = Some("Sammy Corporation");
    codes[0xC0] = Some("Taito");
    codes[0xC2] = Some("Kemco");
    codes[0xC3] = Some("Square");
    codes[0xC4] = Some("Tokuma Shoten");
    codes[0xC5] = Some("Data East");
    codes[0xC6] = Some("Tonkin House");
    codes[0xC8] = Some("Koei");
    codes[0xC9] = Some("UFL");
    codes[0xCA] = Some("Ultra Games");
    codes[0xCB] = Some("VAP, Inc.");
    codes[0xCC] = Some("Use Corporation");
    codes[0xCD] = Some("Meldac");
    codes[0xCE] = Some("Pony Canyon");
    codes[0xCF] = Some("Angel");
    codes[0xD0] = Some("Taito");
    codes[0xD1] = Some("SOFEL (Software Engineering Lab)");
    codes[0xD2] = Some("Quest");
    codes[0xD3] = Some("Sigma Enterprises");
    codes[0xD4] = Some("ASK Kodansha Co.");
    codes[0xD6] = Some("Naxat Soft");
    codes[0xD7] = Some("Copya System");
    codes[0xD9] = Some("Banpresto");
    codes[0xDA] = Some("Tomy");
    codes[0xDB] = Some("LJN");
    codes[0xDD] = Some("Nippon Computer Systems");
    codes[0xDE] = Some("Human Ent.");
    codes[0xDF] = Some("Altron");
    codes[0xE0] = Some("Jaleco");
    codes[0xE1] = Some("Towa Chiki");
    codes[0xE2] = Some("Yutaka");
    codes[0xE3] = Some("Varie");
    codes[0xE5] = Some("Epoch");
    codes[0xE7] = Some("Athena");
    codes[0xE8] = Some("Asmik Ace Entertainment");
    codes[0xE9] = Some("Natsume");
    codes[0xEA] = Some("King Records");
    codes[0xEB] = Some("Atlus");
    codes[0xEC] = Some("Epic/Sony Records");
    codes[0xEE] = Some("IGS");
    codes[0xF0] = Some("A Wave");
    codes[0xF3] = Some("Extreme Entertainment");
    codes[0xFF] = Some("LJN");
    codes
};
static NEW_LICENSE_CODE: [Option<&'static str>; 0xA5] = {
    let mut codes = [None; 0xA5];
    codes[0x00] = Some("None");
    codes[0x01] = Some("Nintendo Research & Development 1");
    codes[0x08] = Some("Capcom");
    codes[0x13] = Some("EA (Electronic Arts)");
    codes[0x18] = Some("Hudson Soft");
    codes[0x19] = Some("B-AI");
    codes[0x20] = Some("KSS");
    codes[0x22] = Some("Planning Office WADA");
    codes[0x24] = Some("PCM Complete");
    codes[0x25] = Some("San-X");
    codes[0x28] = Some("Kemco");
    codes[0x29] = Some("SETA Corporation");
    codes[0x30] = Some("Viacom");
    codes[0x31] = Some("Nintendo");
    codes[0x32] = Some("Bandai");
    codes[0x33] = Some("Ocean Software/Acclaim Entertainment");
    codes[0x34] = Some("Konami");
    codes[0x35] = Some("HectorSoft");
    codes[0x37] = Some("Taito");
    codes[0x38] = Some("Hudson Soft");
    codes[0x39] = Some("Banpresto");
    codes[0x41] = Some("Ubi Soft");
    codes[0x42] = Some("Atlus");
    codes[0x44] = Some("Malibu Interactive");
    codes[0x46] = Some("Angel");
    codes[0x47] = Some("Bullet-Proof Software");
    codes[0x49] = Some("Irem");
    codes[0x50] = Some("Absolute");
    codes[0x51] = Some("Acclaim Entertainment");
    codes[0x52] = Some("Activision");
    codes[0x53] = Some("Sammy USA Corporation");
    codes[0x54] = Some("Konami");
    codes[0x55] = Some("Hi Tech Expressions");
    codes[0x56] = Some("LJN");
    codes[0x57] = Some("Matchbox");
    codes[0x58] = Some("Mattel");
    codes[0x59] = Some("Milton Bradley Company");
    codes[0x60] = Some("Titus Interactive");
    codes[0x61] = Some("Virgin Games Ltd.");
    codes[0x64] = Some("Lucasfilm Games");
    codes[0x67] = Some("Ocean Software");
    codes[0x69] = Some("EA (Electronic Arts)");
    codes[0x70] = Some("Infogrames");
    codes[0x71] = Some("Interplay Entertainment");
    codes[0x72] = Some("Broderbund");
    codes[0x73] = Some("Sculptured Software");
    codes[0x75] = Some("The Sales Curve Limited");
    codes[0x78] = Some("THQ");
    codes[0x79] = Some("Accolade");
    codes[0x80] = Some("Misawa Entertainment");
    codes[0x83] = Some("lozc");
    codes[0x86] = Some("Tokuma Shoten");
    codes[0x87] = Some("Tsukuda Original");
    codes[0x91] = Some("Chunsoft Co.");
    codes[0x92] = Some("Video System");
    codes[0x93] = Some("Ocean Software/Acclaim Entertainment");
    codes[0x95] = Some("Varie");
    codes[0x96] = Some("Yonezawa/s'pal");
    codes[0x97] = Some("Kaneko");
    codes[0x99] = Some("Pack-In-Video");
    codes[0xA4] = Some("Konami (Yu-Gi-Oh!)");

    codes
};
static CARTRIDGE_TYPES: [Option<&'static str>; 0x100] = {
    let mut types = [None; 0x100];
    types[0x00] = Some("ROM ONLY");
    types[0x01] = Some("MBC1");
    types[0x02] = Some("MBC1+RAM");
    types[0x03] = Some("MBC1+RAM+BATTERY");
    types[0x05] = Some("MBC2");
    types[0x06] = Some("MBC2+BATTERY");
    types[0x08] = Some("ROM+RAM");
    types[0x09] = Some("ROM+RAM+BATTERY");
    types[0x0B] = Some("MMM01");
    types[0x0C] = Some("MMM01+RAM");
    types[0x0D] = Some("MMM01+RAM+BATTERY");
    types[0x0F] = Some("MBC3+TIMER+BATTERY");
    types[0x10] = Some("MBC3+TIMER+RAM+BATTERY");
    types[0x11] = Some("MBC3");
    types[0x12] = Some("MBC3+RAM");
    types[0x13] = Some("MBC3+RAM+BATTERY");
    types[0x19] = Some("MBC5");
    types[0x1A] = Some("MBC5+RAM");
    types[0x1B] = Some("MBC5+RAM+BATTERY");
    types[0x1C] = Some("MBC5+RUMBLE");
    types[0x1D] = Some("MBC5+RUMBLE+RAM");
    types[0x1E] = Some("MBC5+RUMBLE+RAM+BATTERY");
    types[0x20] = Some("MBC6");
    types[0x22] = Some("MBC7+SENSOR+RUMBLE+RAM+BATTERY");
    types[0xFC] = Some("POCKET CAMERA");
    types[0xFD] = Some("BANDAI TAMA5");
    types[0xFE] = Some("HuC3");
    types[0xFF] = Some("HuC1+RAM+BATTERY");

    types
};
static RAM_TYPES: [Option<&'static str>; 0x06] = {
    let mut types = [None; 0x06];
    types[0x00] = Some("None");
    types[0x02] = Some("8KB");
    types[0x03] = Some("32KB");
    types[0x04] = Some("128KB");
    types[0x05] = Some("64KB");

    types
};

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct CartridgeHeader {
    pub entry: [u8; 4],           // 0x100 - 0x103
    pub logo: [u8; 48],           // 0x104 - 0x133
    title: [u8; 15],              // 0x134 - 0x142
    pub cgb_flag: u8,             // 0x143
    new_licensee_code: [u8; 2],   // 0x144 - 0x145
    pub sgb_flag: u8,             // 0x146
    pub cartridge_type: u8,       // 0x147
    pub rom_size: u8,             // 0x148
    pub ram_size: u8,             // 0x149
    pub destination_code: u8,     // 0x14A
    old_licensee_code: u8,        // 0x14B
    pub rom_version: u8,          // 0x14C
    pub header_checksum: u8,      // 0x14D
    pub global_checksum: [u8; 2], // 0x14E - 0x14F
}

impl CartridgeHeader {
    pub fn get_title(&self) -> String {
        String::from_utf8_lossy(&self.title)
            .trim_end_matches('\0')
            .to_string()
    }

    pub fn get_new_licensee_code(&self) -> u16 {
        u16::from_be_bytes(self.new_licensee_code)
    }

    pub fn get_licensee_name(&self) -> &'static str {
        if self.old_licensee_code == 0x33 {
            NEW_LICENSE_CODE[self.get_new_licensee_code() as usize].unwrap_or("Unknown")
        } else {
            OLD_LICENSE_CODE[self.old_licensee_code as usize].unwrap_or("Unknown")
        }
    }

    pub fn get_cartridge_type(&self) -> &'static str {
        CARTRIDGE_TYPES[self.cartridge_type as usize].unwrap_or("Unknown")
    }

    pub fn get_ram_size(&self) -> &'static str {
        RAM_TYPES[self.ram_size as usize].unwrap_or("Unknown")
    }
}

pub fn load_header(rom: &[u8]) -> CartridgeHeader {
    let header_slice = &rom[0x100..0x150];

    unsafe { std::ptr::read(header_slice.as_ptr() as *const CartridgeHeader) }
}
