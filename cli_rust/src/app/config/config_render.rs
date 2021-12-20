pub struct ConfigRender {
    pub menu_titles: Vec<String>,
    pub height_header: u16,
    pub min_height_main: u16,
    pub height_footer: u16,
}

impl ConfigRender {

    pub fn new()-> ConfigRender {
        ConfigRender {
            menu_titles: vec![
                String::from("Inicio"),
                String::from("Ayuda"),
                String::from("Salir")
            ],
            height_header: 3,
            min_height_main: 6,
            height_footer: 3,
        }
    }

    pub fn get_main_titles(& self)->&Vec<String> {
        &self.menu_titles
    }
}
