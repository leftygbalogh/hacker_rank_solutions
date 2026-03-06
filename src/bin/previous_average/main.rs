mod proj_template;

#[path = "../util/screen.rs"]
mod screen;

fn main(){
	screen::clear();
	proj_template::printer(43);
}
