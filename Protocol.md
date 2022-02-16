# Legend

# 12-Feb-2022

* Now continuing with this

* Late last year I set up a basic foundation for the Kanji Tree together with Robert

* Thus far what I have is a relatively lightweight library that reads Kanji from a `kanji.json` file and saves them in a vector, then uses that vector to look up Kanji

* Thus far, it can do three things:

  * Get a Kanji via its identifier
  * Get a Kanji's parents via the Kanji's identifier
  * Get a Kanji's children via the Kanji's identifier

* Robert came up with that data structure, and I am really happy with it thus far

  * The fact that all the Kanji exist separately from one another in a dictionary-like structure means that I won't have to worry about any tight couplings
  * The only potential issue would be if Kanji were to be moved, but since the Kanji tree is by definition a pretty static structure based on the Japanese language, which is not at all likely to change

* To formally declare the purpose of this project again here:

  * The purpose of the Kanji Tree is to have a graphical representation of which Kanji are contained in other Kanji, for example: (日&立) -> 音 -> 闇 <- 門
  * The ultimate goal would be to have this in a flowchart-like structure, but as a first goal, I would like a view that displays a Kanji, together with its parents and children
  * Clicking on a parent or child should then open the view for that Kanji

* However, an absolute must-have for this is for there to be some sort of GUI

* I have by now finished the Rust Book (https://doc.rust-lang.org/stable/book/), which covered not only basic but already fairly advanced backend topics

  * However, there was no talk in there whatsoever of frontend topics, and an initial search already made it clear that frontend is not one of Rust's strong suits
  * And yet, there seem to be at least a few potential candidates

* Anyway, first I clearly need some sort of GUI framework or crate

  * This seems like a good place to start from: https://www.libhunt.com/l/rust/topic/gui

  * Also, I asked in the rust discord chat, and this was recommended to me:

    * https://crates.io/crates/egui

    * This is the sample page for egui, and I must say, it looks really, really good

      * https://www.egui.rs/index.html

    * The documentation for egui is here:

      * https://docs.rs/egui/latest/egui/

    * One thing I will want to **keep in mind** is this:

      * Can I use `egui` with non-latin characters?

        Yes! But you need to install your own font (`.ttf` or `.otf`) using `Context::set_fonts`.

    * I now cloned the following sample project from git

      * https://github.com/emilk/eframe_template/

      * Looks like this is supposed to be used as a template for building an application on, but I already have the kanji-tree project all set up

      * I should probably try and play around with the sample project for a bit

      * First, I'll want to create a field there in the main window that displays the current kanji

      * The first thing I tried is already not so good

      * I tried creating a "kanji" variable and have it displayed in a label, but when I try that I get the following error:

        * ````
          error[E0277]: the trait bound `WidgetText: From<&mut String>` is not satisfied
             --> src\app.rs:113:16
              |
          113 |             ui.label(kanji);
              |                ^^^^^ the trait `From<&mut String>` is not implemented for `WidgetText`
              |
              = help: the following implementations were found:
                        <WidgetText as From<&String>>
                        <WidgetText as From<&str>>
                        <WidgetText as From<Arc<eframe::egui::Galley>>>
                        <WidgetText as From<String>>
                      and 2 others
              = note: required because of the requirements on the impl of `Into<WidgetText>` for `&mut String`
          ````

        * Turns out I needed to say `&*kanji` here. Still no idea as to why, but at least it works

        * Now, I have a field that displays a `☐` instead, I suppose that's where the aforementioned font support comes in

        * It says that I need to install my own font using `Context::set_fonts`, but how do I do that, and where?

        * This looks like it might help:

          * https://users.rust-lang.org/t/is-posible-egui-change-fonts-to-japanese-how/59662/4

          * Nope, that doesn't really seem to help after all...

          * Even if I just cut and paste the whole thing, I still get the following error:

            * ``````rust
              error[E0308]: mismatched types
                --> src\app.rs:41:50
                 |
              41 |         font.font_data.insert("mPlus".to_owned(),std::borrow::Cow::Borrowed(include_bytes!("../fonts/msgothic.ttc")));
                 |                                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `FontData`, found enum `Cow`
                 |
                 = note: expected struct `FontData`
                              found enum `Cow<'_, [u8; 9176636]>`
              ``````

          * Anyway, here's how I got it to work in the end:

            * ``````
                      let mut font = FontDefinitions::default();
                      font.font_data.insert(
                          keys::FONT_GOTHIC.to_string(),
                          FontData{
                              font: std::borrow::Cow::Borrowed(
                                  include_bytes!("../fonts/msgothic.ttc")
                              ),
                              index: 0
                          }
                      );
                      font.fonts_for_family.get_mut(&FontFamily::Monospace).unwrap().insert(0, keys::FONT_GOTHIC.to_string());
                      font.fonts_for_family.get_mut(&FontFamily::Proportional).unwrap().insert(0, keys::FONT_GOTHIC.to_string());
                      _ctx.set_fonts(font);
              ``````

  * With that, I now managed to display my first Kanji in the test project

* This is as far as I'm getting today



# 16-Feb-2022

* Now continuing with this

* Last time I played around with egui for creating a rust interface, and got some barely tolerable results

* Today, I shall try to actually get started with the KanjiTree GUI here

* Presently, I am stuck on getting my main class recognize a class in another file in the same directory

  * This is particularly weird since this just happens to work in the egui sample project, but the exact same thing doesn't seem to work for me
  * I got some help from the rust chat for this one
  * Apparently, for the `main.rs` to recognize the `app.rs`, there needs to be a `pub use app::KanjiTreeApp;` directive in the `lib.rs`
  * Anyway, now this works

* It wasn't exactly straightforward after that, but eventually I still managed to get a first window that displayed a title and a kanji

* Let that be Version 0-1-0

* Before anything else, I want to play around with the layout a bit

  * Right now, the active kanji is displayed on the top left, and is rather small

  * I want it to be in the centered and big

    * The "big" thing is actually really, really important, since at the current size of the kanji, it is going to be extremely difficult to make out some of the miniscule differences of the more complex kanji

  * Time to hit the egui reference for answers

  * ...what is the egui reference?

  * I found this. Let's see if it holds the answers that I seek

    * https://docs.rs/egui/latest/egui/
    * ...regrettably, I can't find anything there

  * The `eframe_template` doesn't seem to have anything like that either

  * I have now downloaded the `eframe` repository with more samples, and I am still looking

  * I thing it may have something to do with the number displayed behind these:

    * ``````
          job.append(
              "mixing ",
              0.0,
              TextFormat {
                  font_id: FontId::proportional(20.0),
                  color: default_color,
                  ..Default::default()
              },
          );
          job.append(
              "fonts, ",
              0.0,
              TextFormat {
                  font_id: FontId::monospace(14.0),
                  color: default_color,
                  ..Default::default()
              },
          );
      ``````

  * And there's also something like this mentioned in here:

    * https://docs.rs/egui/latest/egui/struct.FontDefinitions.html

  * ...however, there's nothing like a simple "how to" for setting a font size

  * And also, I am importing my own font here, so that might make it more difficult

  * I think I found something like that here now:

    * ```
      pub struct FontId {
          /// Height in points.
          pub size: f32,
      
          /// What font family to use.
          pub family: FontFamily,
          // TODO: weight (bold), italics, …
      }
      ```

  * ...however, it would seem that this is not yet live, and I have no idea when or if this might get released, so it means that I can't use that

  * That sucks

  * I wonder if there's any other way to do font size in egui

  * Based on what I read, there really isn't

  * That's a shame. It means I can't use egui after all

* What are alternatives?

  * Iced
    * https://github.com/iced-rs/iced

  * Tuix
    * https://rustrepo.com/repo/geom3trik-tuix
  * Druid
    * https://github.com/linebender/druid
  * Azul
    * https://rust.libhunt.com/azul-alternatives
  * Orbtk
    * https://github.com/redox-os/orbtk

* Since Iced seems to be pretty popular, I'll have a look at that now

  * The documentation for iced is here:
    * https://docs.rs/iced/latest/iced/
  * Examples for iced are here:
    * https://github.com/iced-rs/iced/tree/master/examples
    * These samples include all sorts of cool stuff that might be useful, including a Pokédex which displays random Pokémon entries, complete with pictures
      * I think I'll analyze that one, since it seems to be closely related to what I want to do with my Kanji tree
      * Most notably, I'm pretty certain this one uses different font sizes (and not just pre-programmed font styles)
    * It'll probably be better to start with a smaller sample though
    * The `counter` sample seems pretty minimal
  * I am working on this now, and making progress, but I am still running into trouble
  * But I managed to push through, and now built the first working iced app, that does not display my kanji due to an incompatible font, but in a nice, big size
  * So, I suppose I have to figure out how to make fonts work in this one too
    * I think the Pokédex sample featured different fonts
    * ...or not
    * 

* 

# ⚓