extern crate slack;

use slack::{Event, RtmClient};


struct BotHandler<'bot_handler> {
    ch_name: &'bot_handler str,
}

impl <'bot_handler> BotHandler<'bot_handler> {
    fn set_channel_name(&mut self, ch_name: &'bot_handler str) {
        self.ch_name = ch_name;
    }
    fn get_channel_id (&mut self, channels: &Vec<slack::Channel>, ch_name: &str) -> Option<String>{
        channels.iter().find(|ch| match ch.name {
            None => false,
            Some(ref name) => name == ch_name,
        }).and_then(|ch| ch.id.clone())
            .and_then(|ch_id| {
                let res: String = ch_id;
                Some(res)
            } )
    }
}

impl <'bot_handler> slack::EventHandler for BotHandler<'bot_handler> {
    fn on_event(&mut self, cli: &RtmClient, event: Event) {
        println!("callback: on_event Event: {:?}", event);
    }

    fn on_close(&mut self, cli: &RtmClient) {
        println!("callback: on_close");
    }

    fn on_connect(&mut self, cli: &RtmClient) {
        println!("callback: on_connect");

        let channels = cli.start_response().channels
            .as_ref()
            .expect("Could not get the list of channels.");

        self.get_channel_id(channels, self.ch_name)
            .and_then(|ch_id| {
                let res = cli.sender().send_message(&ch_id, "Hello, world!");
                match res {
                    Ok(_) => Some(ch_id),
                    Err(err) => {
                        println!("send_message() failed with err = {:?}", err);
                        None
                    }
                }
            });
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let session_ctx = match args.len() {
        0 | 1 | 2 => panic!("Too few arguments(expected 2). \n `./cargo run -- [channel_name] [api-key]`"),
        x => (args[x - 2].clone(), args[x - 1].clone()),
    };

    let mut handler = BotHandler{ ch_name: "", };
    handler.set_channel_name(&session_ctx.0);
    let rclient = RtmClient::login_and_run(&session_ctx.1, &mut handler);
    match rclient {
        Ok(_) => {},
        Err(err) => panic!("Client create ERROR: {}.", err),
    };
}
