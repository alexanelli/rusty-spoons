use std::thread;
use std::time::Duration;
use wasm_bindgen_futures::spawn_local;

struct Game {
  //Increment this by one every click
  clicks: i32,

  //autmoatically click this many times per second
  auto_clicks: i32,

  //cost of the upgrades, increases exponentially
  cost: i32,

  //level of the speed up upgrade
  upgrade_speed: i32,

  //ms between each autoclick
  click_rate: i32,
  
  //storing interval here so we can update it
  interval_auto: i32,

  //clicks per physical click
  click_increment: i32,
}

pub fn build_game() -> Game {
  Game{
    clicks: 0,
    auto_clicks: 0,
    cost: 1,
    upgrade_speed: 0,
    click_rate: 1000,
    click_increment: 1,
    interval_auto: 1,
  };
}

impl Game {

  pub async fn start_game_loop(&self) {
    spawn_local(async {
      thread::spawn(|| {
        loop {
          thread::sleep(Duration::from_millis(self.click_rate));
        }
      });
    });
  }

  pub fn main_click_btn_handler(&self) {
    self.clicks += self.click_increment;
  }
  fn cant_afford(&self, cost: i32) -> bool{
    println!("you can't afford this");
    return self.clicks < cost;
  }

  pub fn buy_click_btn_handler(&self) {
    if cant_afford(self.cost) { return; }

    self.clicks -= self.cost;
    self.auto_clicks += 1;
    self.cost = i32::pow(2, self.auto_clicks);

  }

  pub fn upgrade_speed_btn_handler(&self) {
    let upgrade_cost = i32::pow(3, self.upgrade_speed) * 100;
    if cant_afford(upgrade_cost){ return; }

    self.clicks -= self.upgrade_cost;
    self.upgrade_speed += 1;
    self.click_rate = self.click_rate * 0.90;
  }

  pub fn increase_click_btn_handler(&self) {
    let upgrade_cost = i32::pow(3, self.click_increment);
    if cant_afford(upgrade_cost){ return; }
    click_increment += 1;
  }

  // pub fn set_storage(&self){
  // //use gloo-storage LocalStorage
  // }

  // pub fn get_storage(&self){

  // }
}

