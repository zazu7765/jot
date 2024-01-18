use chrono::NaiveDate;
use crate::Note;

pub(crate) fn filter_by_date(notes: & mut Vec<Note>, start_date: &str, end_date: &str){
    notes
        .retain(|note|
            if let Ok(note_date) = NaiveDate::parse_from_str(&note.date, "%d/%m/%y"){
                let start = NaiveDate::parse_from_str(start_date, "%d/%m/%y").ok();
                let end = NaiveDate::parse_from_str(end_date, "%d/%m/%y").ok();
                match (start, end) {
                    (Some(start), Some(end)) => note_date >= start && note_date <= end,
                    (Some(start), None) => note_date >= start,
                    (None, Some(end)) => note_date <= end,
                    (None, None) => true,
                }
            }
            else{
                false
            }
        )

}

pub(crate) fn filter_by_tag( notes: &mut Vec<Note>){
    notes.sort_by(|x, y| x.tag.cmp(&y.tag))
}