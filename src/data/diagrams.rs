use super::data_read::{line_managers, practice_leads, team_members};

pub async fn draw_diagrams() -> String {
    let practice_leads = practice_leads().await;
    let line_managers = line_managers().await;
    let team_members = team_members().await;
    let mut graphviz_code = String::from("Digraph {\nrankdir=TB;\n");

    for lead in &practice_leads {
        graphviz_code.push_str(&format!(
            "\"{}\" [shape=box, label=\"{},\n {},\n {}\"];\n",
            lead.name, lead.name, lead.role, lead.office
        ));
    }

    for manager in &line_managers {
        graphviz_code.push_str(&format!(
            "\"{}\" [shape=box, label=\"{},\n {},\n {}\"];\n",
            manager.name, manager.name, manager.role, manager.office
        ));
        graphviz_code.push_str(&format!(
            "\"{}\" -> \"{}\" [weight=10];\n",
            manager.manager, manager.name
        ));

        let mut team_record_label = String::from("{");
        let mut first_member = true;
        for member in &team_members {
            if member.manager == manager.name {
                if first_member {
                    team_record_label.push_str(&format!(
                        "{}\\n {}\\n{} ",
                        member.name, member.role, member.office
                    ));
                    first_member = false;
                } else {
                    team_record_label.push_str(&format!(
                        "|{}\\n {}\\n {} ",
                        member.name, member.role, member.office
                    ));
                }
            }
        }
        team_record_label.push('}');
        let team_node_name = manager.name.replace(' ', "_");
        graphviz_code.push_str(&format!(
            "\"{}\" [shape=record, label=\"{}\"];\n",
            team_node_name, team_record_label
        ));
        graphviz_code.push_str(&format!(
            "\"{}\" -> \"{}\";\n",
            manager.name, team_node_name
        ));
    }

    graphviz_code.push_str("}\n");
    graphviz_code
}
