use crate::config;
use config::Color;

use super::creator_trait;

pub struct TableCreator {}

impl creator_trait::Creator for TableCreator {
    fn builder(vec: Vec<Vec<Color>>) -> String {
        let mut s = String::from("");
        for _r in vec {
            let mut row = String::from("");
            for _c in _r {
                let tb = builder_td(if _c == Color::Black { " B " } else { " W " });
                row = row + &tb;
            }
            s = s + &builder_tr(&row);
        }
        builder_table(&builder_tbody(&s))
    }
}

fn builder_table(content: &str) -> String {
    let base_str = String::from("<table>");
    base_str + content + "</table>"
}

fn builder_tbody(content: &str) -> String {
    let base_str = String::from("<tbody>");
    base_str + content + "</tbody>"
}

fn builder_tr(content: &str) -> String {
    let base_str = String::from("<tr>");
    base_str + content + "</tr>"
}

fn builder_td(content: &str) -> String {
    let base_str = String::from("<td>");
    base_str + content + "</td>"
}

#[cfg(test)]
mod test {
    use crate::config::Color;
    use crate::creator::creator_trait::Creator;
    use crate::creator::table_creator::TableCreator;

    #[test]
    fn test_target_cells() {
        // Test
        let v = vec![
            vec![Color::Black, Color::White],
            vec![Color::Black, Color::White],
        ];
        println!("{}", &TableCreator::builder(v))
    }
}
