use printpdf::*;
use std::fs::File;
use std::io::{BufWriter};
use text_io::read;

fn calculate_average(total_marks: f32, subjects: u32) -> f32 {
    total_marks / subjects as f32
}

fn assign_grade(average: f32) -> &'static str {
    match average {
        x if x >= 90.0 => "A",
        x if x >= 75.0 => "B",
        x if x >= 60.0 => "C",
        _ => "D",
    }
}

fn generate_pdf(name: &str, total: f32, subjects: u32, average: f32, grade: &str) {
    let (doc, page1, layer1) = PdfDocument::new("Report Card", Mm(210.0), Mm(297.0), "Layer 1");

    let current_layer = doc.get_page(page1).get_layer(layer1);
    let font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();

    let text = format!(
        "Report Card\n\nName: {}\nSubjects: {}\nTotal Marks: {}\nAverage: {:.2}\nGrade: {}",
        name, subjects, total, average, grade
    );

    current_layer.use_text(text, 14.0, Mm(20.0), Mm(250.0), &font);

    doc.save(&mut BufWriter::new(File::create("report_card.pdf").unwrap())).unwrap();
}

fn main() {
    println!("Enter student name:");
    let name: String = read!("{}\n");

    println!("Enter number of subjects:");
    let subjects: u32 = read!();

    println!("Enter total marks:");
    let total_marks: f32 = read!();

    let average = calculate_average(total_marks, subjects);
    let grade = assign_grade(average);

    println!("\n--- Student Report Card ---");
    println!("Name: {}", name);
    println!("Subjects: {}", subjects);
    println!("Total Marks: {}", total_marks);
    println!("Average: {:.2}", average);
    println!("Grade: {}", grade);

    generate_pdf(&name, total_marks, subjects, average, grade);
    println!("\nPDF generated: report_card.pdf");
}
