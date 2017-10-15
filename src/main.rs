extern crate clap;
#[macro_use] extern crate error_chain;
extern crate html5ever;
extern crate scraper;
#[macro_use] extern crate string_cache;
extern crate tendril;
use std::str::FromStr;

mod args_and_usage;

use args_and_usage::parse_args;

use html5ever::{ParseOpts, parse_document, parse_fragment};
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::rcdom::RcDom;
use html5ever::rcdom::NodeEnum::Element;
use html5ever::serialize::{SerializeOpts, serialize, TraversalScope};
use html5ever::tendril::TendrilSink;
use tendril::Tendril;

use scraper::{Html, Selector};
use scraper::element_ref::ElementRef;
use std::fs;
use std::fs::File;
use std::io::Read;

quick_main!(|| -> Result<()> {
    let args = parse_args();

    // The first step is to move the original file aside
    // to do that we need to consuct the aside path
    let mut original_path = args.input.clone();
    if ! original_path.set_extension("aside") {
        bail!(ErrorKind::UnableToAddAsideExtension);
    }

    // Now rename input
    fs::rename(&args.input, &original_path)
        .chain_err(|| format!(
                "Unable to rename\n{}\nto\n{}",
                args.input.display(),
                original_path.display()
            )
        )?;

    // Now lets get the input file handle read
    let mut input_file = File::open(&original_path)
        .chain_err(|| format!("Can't open input file: {}", original_path.display()))?;

    // Now lets have the output handle ready to go
    // Which happens to be same as input path was
    let mut output_file = File::create(&args.input)
        .chain_err(|| format!("Can't create output file: {}", args.input.display()))?;

    // Read the input_file into a buffer
    let mut input_buffer = String::new();
    input_file.read_to_string(&mut input_buffer)
        .chain_err(|| format!("Can't read from input file: {}", original_path.display()))?;

/*    let mut fragment = Html::parse_fragment(&input_buffer);

    let paragraph_selecter = Selector::parse("p").unwrap();
    let math_selector = Selector::parse(".display").unwrap();

    for p in fragment.select(&paragraph_selecter) {
        let inards: Vec<ElementRef>= p.select(&math_selector).collect();
        if inards.len() == 1 {
            println!("Found ex: {}", inards[0].html());
        }
    }

    {
        let handle = fragment.get_document();
        println!("handle: {:?}", handle);
        let e = fragment.tree.get(handle);
        println!("node: {:?}", e);
        let e1 = ElementRef::wrap(e).unwrap();
        println!("elementref: {:?}", e1);
        //output_file.write_all(e1.html().as_bytes());

        serialize(&mut output_file, &e1 , Default::default())?;
    }
*/

	let opts = ParseOpts {
		tree_builder: TreeBuilderOpts {
			drop_doctype: true,
			..Default::default()
		},
		..Default::default()
	};

    let mut dom = parse_fragment(
		RcDom::default(),
		opts,
		qualname!(html, "body"),
		Vec::new()
		)
        .from_utf8()
        .read_from(&mut input_buffer.as_bytes())
        .unwrap();
{
	let mut document = dom.document.borrow_mut();
	let mut fragment = document.children[0].borrow_mut();
	
	let div_name = qualname!(html, "div");
	let p_name = qualname!(html, "p");
	let span_name = qualname!(html, "span");
	let attr_tendril = Tendril::from_str("math display")
		.expect("Can't make tendril");

	let mut paragraphs_to_check = Vec::new();

	for index in 0..fragment.children.len() {
		let child = fragment.children[index].borrow();
		
		if let Element(ref qual_name, _, _) = child.node {
			if &p_name == qual_name {
				println!("index: {}, found p", index);
				paragraphs_to_check.push(fragment.children[index].clone());	
			} else {
				println!("index: {}, no p, {:?}", index, qual_name);
			}
		}
	}

	for p in paragraphs_to_check {
		let mut para = p.borrow_mut();

		let mut swap_flag = false;

		for index in 0..para.children.len() {
			let child = para.children[index].borrow();

			if let Element(ref qual_name, _, ref attributes) = child.node {
				if qual_name != &span_name {
					continue;	
					println!("p child {} had {:?}", index, qual_name);
				}

				for attr in attributes {
					if attr.value == attr_tendril {
						swap_flag = true;
						println!("p child {}, had right attr", index);
					} else {
						println!("p child {}, had wrong attr {:?}", index, attr);
					}
				}
			}
		}

		if swap_flag {
			if let Element(ref mut qual_name, _, _) = para.node {
				*qual_name = div_name.clone();
			}
		}
	}
}	
	let serialize_opts = SerializeOpts {
		scripting_enabled: true,
		traversal_scope: TraversalScope::ChildrenOnly
	};
    serialize(
		&mut output_file,
		&dom.document.borrow().children[0],
		serialize_opts
	).chain_err(|| format!("Unable to serialize fragment"))?;

// https://stackoverflow.com/questions/38859811/how-do-i-parse-a-page-with-html5ever-modify-the-dom-and-serialize-it
    Ok(())
});

error_chain! {
    errors {
        UnableToAddAsideExtension {
            description("We were unable to add the .aside extension the ouput path")
        }
        UnableToParseSelector {
            description("Unable to parse selector")
        }
    }

    foreign_links {
        IO(std::io::Error);
    }
}
