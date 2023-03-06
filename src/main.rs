/*  BaaMeows 2023
this program literally just grabs a frame from whatever camera happens
to be at index zero and shoves it into a window 60 times a second.
(so far?)
thats it. thats literally the thing. the whole thing. 
also it says uwu and ywy and :3 */
// :3 
/* escapi is a really simple camera capture library that uses
an ancient version of WinAPI. its just kind of. there.
I don't think that its cross-platform???????
we should replace it (later)
rust is mad at me :c */
use escapi;
/* fltk for rust me when fltk for rust its fltk for rust
it took me so fucking long to figure out how to update
a window 60 times a second it is insane
(the answer was timeout3())
okay we never actually hit 60fps in practice but whatever uh- */
use fltk::{app, enums, frame::Frame, window::Window, prelude::*};
// real (i LOVE stds!!!!)
use std::time::SystemTime;
fn main() {
    /* shove the number of cameras directly into the 
    shell for our viewing pleasure.
    even though we just grab the camera at index
    0 right now and ignore everything else.
    ...still feels cool.... i guess.
    at least it'll tell us if there are zero cameras.,, */
    println!("devices: {}", escapi::num_devices());
    /* the resolution that we're grabbing from the camera
    escapi automatically scales it to whatever we want,
    but it should be the correct aspect ratio, at least */
    const WIDTH: u32 = 960;
    const HEIGHT: u32 = 540;
    // initializing the camera(s?). this should be binocular at some point but uh
    let camera = escapi::init(0, 
        WIDTH, HEIGHT, // 
        60).expect("Could not initialize the camera");
    // tell us all about the said initialized camera and how turned on it is. in like. a definitely not horny way.
    println!("capture initialized, device name: {}", camera.name());
    // set up the app and window and tidbits and things
    let app = app::App::default();
    let mut wind = Window::default()
        .with_size(WIDTH.try_into().unwrap(), HEIGHT.try_into().unwrap()) // WHY ARE THEY SIGNED. ITS A SIZE.
        .center_screen()                                                                // WHEN THE FUCK WILL IT BE NEGATIVE??? FUCK OFF
        .with_label("Hewwo :3 tee hee"); // wow i am so fucking cool
    let mut frame = Frame::default().size_of(&wind);
    wind.make_resizable(false);
    wind.end();
    wind.show();
    /* instead of like, an actual loop, we're using timeouts! (bleh?)
    every time the timeout is called the window is allowed
    to update, so if we just have a timeout that keeps calling
    itself we now have a loop that updates the window every time
    its done.
    this could have been multithreaded instead. I would rather die. */
    app::add_timeout3(0.0, move |handle| { // time is set to zero because why would time not be set to zero
        // get the time at the start of the frame so we can get the frame time later
        let start_time = SystemTime::now();
        // grab the current frame from the camera
        let pixels = camera.capture().expect("Could not capture an image");
        /* take the raw pixels from the camera and make them into vector from despicable me.
        Honestly? I barely know how this part works. It *looks* like its taking each pixel
        and splitting it into its RGB components and like. uh,,. putting them into a 2D array????
        that. makes sense. I guess.
        it was in the escapi example and it breaks without it. so like.?? cool. great. awesome.
        who doesn't like a nice image buffer. delicious.
        */
        let mut buffer = vec![0; WIDTH as usize * HEIGHT as usize * 3];
        for i in 0..pixels.len() / 4 {
            buffer[i * 3] = pixels[i * 4 + 2];
            buffer[i * 3 + 1] = pixels[i * 4 + 1];
            buffer[i * 3 + 2] = pixels[i * 4];
        }
        // take the buffer and turn it into an fltk RgbImage (WHYYYYY) so that it can be displayed
        let image = fltk::image::RgbImage::new( // you monsters. the horror. what would the image libary think of this. its crying right now.
            &buffer,
            WIDTH.try_into().unwrap(), HEIGHT.try_into().unwrap(), /* I’m killing you. I’m killing you. I don’t care about anything else, 
            I don’t give a shit about anything else, I- My programming is just “GET THAT FUCKING GUY RIGHT NOW”. It doesn’t- There’s no, like, “Oh, he’s running? 
            Oh, back off a little!”, it’s just **THUMP** **THUMP THUMP** until I get you. */
            enums::ColorDepth::Rgb8 // oh right also we just use Rgb8 because whatever
        ).unwrap(); // like a present!
        // draw the image and update the window
        frame.set_image(Some(image)); // SOME IMAGE, EY???? this language is cool
        frame.redraw(); // yeag
        // set the title of the window to how long this frame took
        // (we assume that the system clock has not gone backwards)
        let frame_time = SystemTime::now().duration_since(start_time);
        wind.set_label(&format!("{frame_time:?} uwu")); // me when im silly
        // looooooooooooop
        app::repeat_timeout3(0.0, handle);
    });
    // my goodness
    app.run().unwrap();
}
/*
                      _                        
                      \`*-.                    
                       )  _`-.                 
                      .  : `. .                
                      : _   '  \               
                      ; *` _.   `*-._          
                      `-.-'          `-.       
                        ;       `       `.     
                        :.       .        \    
                        . \  .   :   .-'   .   
                        '  `+.;  ;  '      :   
                        :  '  |    ;       ;-. 
                        ; '   : :`-:     _.`* ;
               [bug] .*' /  .*' ; .*`- +'  `*' 
                     `*-*   `*-*  `*-*'        
*/
