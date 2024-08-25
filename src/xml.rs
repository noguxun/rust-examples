#[test]
fn xml01() {
    // quick-xml, name space stripping
    // https://github.com/tafia/quick-xml/issues/347
    #[derive(Debug, serde::Deserialize)]
    #[serde(rename = "period")]
    struct Period {
        #[serde(rename = "SpliceInfoSection")]
        #[allow(dead_code)]
        splice_info: Vec<SpliceInfoSection>,
    }

    #[derive(Debug, serde::Deserialize)]
    struct SpliceInfoSection {
        #[serde(rename = "@protocolVersion")]
        #[allow(dead_code)]
        protocol_version: String,
    }

    let xml = r#"
        <Period><scte35:SpliceInfoSection protocolVersion="0" ptsAdjustment="183265" tier="4095"></scte35:SpliceInfoSection></Period>
    "#;
    println!("{:#?}", quick_xml::de::from_str::<Period>(xml).unwrap());
}

#[test]
fn xml02() {
    use quick_xml::events::Event;
    use quick_xml::reader::Reader;

    let xml = r#"<a:tag1 att1 = "test" scte35:abc = "test1" > <tag2><!--Test comment-->Test</tag2><tag2>Test 2</tag2></a:tag1>"#;
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut _count = 0;
    let mut txt = Vec::new();
    let mut buf = Vec::new();

    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    loop {
        // NOTE: this is the generic case when we don't know about the input BufRead.
        // when the input is a &str or a &[u8], we don't actually need to use another
        // buffer, we could directly call `reader.read_event()`
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            // exits the loop when reaching end of file
            Ok(Event::Eof) => break,

            Ok(Event::Start(e)) => {
                println!("Start {e:?}");
                match e.name().as_ref() {
                    b"a:tag1" => {
                        println!(
                            "attributes values: {:?}",
                            e.attributes()
                                .map(|a| {
                                    let att = a.unwrap();
                                    (att.key, String::from_utf8(att.value.into_owned()).unwrap())
                                })
                                .collect::<Vec<_>>()
                        );

                        let pos = reader.buffer_position() as usize;
                        println!(
                            "position {pos} {}{}{}{}{}{}",
                            xml.chars().nth(pos - 1).unwrap(),
                            xml.chars().nth(pos).unwrap(),
                            xml.chars().nth(pos + 1).unwrap(),
                            xml.chars().nth(pos + 2).unwrap(),
                            xml.chars().nth(pos + 3).unwrap(),
                            xml.chars().nth(pos + 4).unwrap()
                        );
                    }
                    b"tag2" => _count += 1,
                    _ => (),
                }
            }
            Ok(Event::End(e)) => {
                println!("End {e:?}");
                let pos = reader.buffer_position() as usize;
                println!(
                    "position {pos} {}",
                    xml.chars().nth(pos - 1).unwrap(),
                    //xml.chars().nth(pos).unwrap(),
                );
            }
            Ok(Event::Text(e)) => txt.push(e.unescape().unwrap().into_owned()),

            // There are several other `Event`s we do not consider here
            _ => (),
        }
        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }
}

#[test]
fn xml03() {
    use quick_xml::events::Event;
    use quick_xml::reader::Reader;

    struct Period {
        start: usize,
        end: usize,
    }

    let mut cur_period = Period { start: 0, end: 0 };

    let xml = r#"
        <tag1 >
            <Period ><!--Test comment--> p1 </Period >
            <Period> p2 <k> </k> </Period>
        </tag1>
    "#;

    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);
    let addr_xml_start = xml.as_ptr() as usize;

    let mut count = 0;
    loop {
        match reader.read_event().unwrap() {
            Event::Start(e) if e.name().as_ref() == b"Period" => {
                count += 1;

                let tag_start_offset = e.as_ptr() as usize - addr_xml_start - 1;
                cur_period.start = tag_start_offset;

                // println!("Start {} {} {} ", reader.buffer_position(), tag_len, tag_start_offset);
            }
            Event::End(e) if e.name().as_ref() == b"Period" => {
                //let tag_end_offset = e.as_ptr() as usize - addr_xml_start;
                // cur_period.end = btag_end_offset + e.len();
                cur_period.end = reader.buffer_position() as usize;

                let period_xml = xml.get(cur_period.start..cur_period.end).unwrap();
                println!("|{}|", period_xml);
            }
            //Event::Text(e) => println!("text# {} #text", e.unescape().unwrap().into_owned()),
            Event::Eof => break,
            _ => (),
        }
    }
    assert_eq!(count, 2);
}

#[test]
fn xml04() {
    use quick_xml::events::{BytesStart, Event};
    use quick_xml::reader::Reader;

    let mut reader = Reader::from_str(
        r#"
    <outer>
        <inner>
            <inner></inner>
            <inner/>
            <outer></outer>
            <outer/>
        </inner>
    </outer>
"#,
    );
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();

    let start = BytesStart::new("outer");
    let end = start.to_end().into_owned();

    // First, we read a start event...
    assert_eq!(
        reader.read_event_into(&mut buf).unwrap(),
        Event::Start(start)
    );
    println!("buf {}", std::string::String::from_utf8_lossy(&buf));

    // ...then, we could skip all events to the corresponding end event.
    // This call will correctly handle nested <outer> elements.
    // Note, however, that this method does not handle namespaces.
    reader.read_to_end_into(end.name(), &mut buf).unwrap();
    println!("buf {}", std::string::String::from_utf8_lossy(&buf));

    // At the end we should get an Eof event, because we ate the whole XML
    assert_eq!(reader.read_event_into(&mut buf).unwrap(), Event::Eof);

    println!("buf {}", std::string::String::from_utf8_lossy(&buf));
}

#[test]
fn xml05() {
    use quick_xml::events::{BytesStart, BytesText, Event};
    use quick_xml::{Reader, Writer};
    use std::io::Cursor;

    let xml = r#"
        <root>
            <foo>hello, world! <f1> </f1></foo>
            <bar>42</bar>
        </root>
    "#;

    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut writer = Writer::new(Cursor::new(Vec::new()));

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) if e.name().as_ref() == b"foo" => {
                // replace the text value of the <foo> element
                let text = "hello, rustacean!";
                writer
                    .write_event(Event::Start(BytesStart::new("foo")))
                    .unwrap();
                writer
                    .write_event(Event::Text(BytesText::new(text)))
                    .unwrap();
                writer
                    .write_event(Event::End(BytesStart::new("foo").to_end()))
                    .unwrap();
            }
            Ok(Event::Eof) => break,
            Ok(e) => writer.write_event(e).unwrap(),
            Err(e) => panic!("error at position {}: {:?}", reader.buffer_position(), e),
        }
    }

    let result = writer.into_inner().into_inner();
    let output = std::str::from_utf8(&result).unwrap();
    println!("{}", output);
}

#[test]
fn xml06() {
    // use quick_xml::events::{BytesStart, BytesText, Event};
    use quick_xml::Reader;
    // use std::io::{BufReader, Cursor, Write};

    let xml = r#"
        <root>
            <foo>hello, world! <f1> </f1></foo>
            <bar>42</bar>
        </root>
    "#;

    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);
}

#[test]
fn xml07() {
    use quick_xml::se::Serializer;
    use serde::Serialize;

    #[derive(Debug, PartialEq, Serialize)]
    struct Struct {
        question: String,
        answer: u32,
    }

    let data = Struct {
        question: "The Ultimate Question of Life, the Universe, and Everything".into(),
        answer: 42,
    };

    let mut buffer = String::new();
    let mut ser = Serializer::new(&mut buffer);
    ser.indent(' ', 2);

    data.serialize(ser).unwrap();

    println!("{buffer}");
}
