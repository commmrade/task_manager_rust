use egui::{RichText, Ui};

use super::app::{make_rich_text, MyApp};




impl MyApp {
    pub fn draw_tasks_ui(&mut self, ui : &mut Ui, ctx: &eframe::egui::Context) {
        ctx.set_pixels_per_point(1.4);
        //Heading
        ui.heading(make_rich_text("TODO List", None));
        ui.add_space(30.0);


        let add_new_text = RichText::new("Add new task:").size(16.0);
        ui.label(add_new_text);
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut self.category_input);
            if ui.button("Add new category").clicked() {
                self.add_category(self.category_input.clone());

                self.category_input.clear();
            }
        });
        ui.add_space(20.0);


        // Input handling
        self.handle_input(ui, ctx);

        // Confirm exit window
        self.confirm_exit_win(ui, ctx);
       
        // Task displayer
        ui.separator();
        ui.vertical(|ui| {
            self.display_tasks(ui);
        });

    }
    pub fn draw_auth_ui(&mut self, ui: &mut Ui, ctx: &eframe::egui::Context) {
        ctx.set_pixels_per_point(1.5);
        // Start a centered vertical layout
        ui.vertical_centered(|ui| {
            ui.add_space(ui.available_height() / 3.5);
            ui.label(make_rich_text("Please authorize", None));
    
            // Center the buttons and labels
            if ui.button("Change").clicked() {
                self.blogin = !self.blogin;
                self.login.clear();
                self.password.clear();
            }
    
        
            if self.blogin {
                
                ui.label(make_rich_text("Login", None));
                egui::TextEdit::singleline(&mut self.login).hint_text("Your username").show(ui);
    
                ui.label(make_rich_text("Password", None));
                egui::TextEdit::singleline(&mut self.password).password(true).hint_text("Password").show(ui);
    
                ui.add_space(10.0);
    
          
                let log_btn = ui.button(make_rich_text("Login", 15.0.into()));
    
                if log_btn.clicked() {
                    self.login();
                }
            } else {
              
                ui.label(make_rich_text("Email", None));
                egui::TextEdit::singleline(&mut self.email).hint_text("Email").show(ui);
    
                ui.label(make_rich_text("Login", None));
                egui::TextEdit::singleline(&mut self.login).hint_text("Your username").show(ui);
    
                ui.label(make_rich_text("Password", None));
                egui::TextEdit::singleline(&mut self.password).password(true).hint_text("Password").show(ui);
    
                ui.add_space(10.0);
    
                let reg_btn = ui.button(make_rich_text("Register", 15.0.into()));
                if reg_btn.clicked() {
                    self.register();
                }
            }
        });
    }
    
}