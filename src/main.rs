mod mml_parser;
mod wav_writer;

fn main() {
    //루돌프 사슴코
    let mml = format!("{}{}",
        "o4 l8 g l4 a l8 g l4 e o5 l4 c o4 l4 a l1 g l8 gaga l4 g o5 l4 c o4 l1 b l4 r",
        "l8 f l4 g l8 f l4 dba l1 g l8 gaga l4 ga l1 e l4 r");
    let notes = mml_parser::parse(mml);
    let bpm = 120.0;
    wav_writer::write("Rudolf.wav", notes, bpm);
    
    // 산토끼
    let mml = format!("{}{}",
            "o4 l4 g l8 ee g e l4 cd l8 edce l4 gs",
            "o5 l8 c o4 l8 g o5 l8 c o4 g o5 c o4 g l4 e g l8 d fed l4 c");
    // 한 음씩 Vec<Note>에 추가
    let notes = mml_parser::parse(mml);
    // WAV 파일로 저장
    let bpm = 120.0;
    wav_writer::write("rabbit.wav", notes, bpm);

    // 작은별
    let mml = format!("{}{}{}",
        "o5 l4 ccgg aal2g l4 ffee ddl2c",
        "l4 ggff eel2d l4 ggff eel2d",
        "l4 ccgg aal2g l4 ffee ddl2c"
    );
    let notes = mml_parser::parse(mml);
    let bpm = 120.0;
    wav_writer::write("twinkle_star.wav", notes, bpm);
}
