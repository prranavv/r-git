pub fn parse_tree(content:&Vec<u8>)->String{
    let mut i=0;
    let null_pos = content[i..].iter().position(|x|*x==b'\0').unwrap();
    i+=null_pos;
    i+=1;
    let mut result = String::new();
    while i<content.len(){
        let space = content[i..].iter().position(|x|*x==b' ').unwrap();
        let mode = std::str::from_utf8(&content[i..i+space]).unwrap();
        i+=space;
        i+=1;
        let null_pos = content[i..].iter().position(|x|*x==b'\0').unwrap();
        let file_name = std::str::from_utf8(&content[i..i+null_pos]).unwrap();
        i+=null_pos;
        i+=1;
        let bytes = &content[i..i+20];
        let hex_string =hex::encode(bytes);
        i+=20;
        let obj_type = if mode == "100644"{
            "blob"
        }else{
            "tree"
        };    
        result.push_str(&format!("{} {} {} {}\n",mode,obj_type,hex_string,file_name));
    }
    let result = result.trim_end().to_string();
    result
}