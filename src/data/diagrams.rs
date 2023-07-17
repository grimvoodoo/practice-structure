use crate::models::practice_models::Person;

use super::data_read::{line_managers, practice_leads, team_members};
use std::collections::HashMap;

pub async fn draw_diagrams() -> String {
    let practice_leads = practice_leads().await;
    let line_managers = line_managers().await;
    let team_members = team_members().await;
    let mut graphviz_code = String::from("Digraph {\nrankdir=TB;\n");

    for lead in &practice_leads {
        graphviz_code.push_str(&format!(
            "\"{}\" [shape=box, style=filled, fillcolor=lightblue, label=\"{},\n {},\n {}\"];\n",
            lead.name, lead.name, lead.role, lead.office
        ));
    }

    for manager in &line_managers {
        graphviz_code.push_str(&format!(
            "\"{}\" [shape=box, style=filled, fillcolor=greenyellow, label=\"{},\n {},\n {}\"];\n",
            manager.name, manager.name, manager.role, manager.office
        ));
        graphviz_code.push_str(&format!(
            "\"{}\" -> \"{}\" [weight=10];\n",
            manager.manager, manager.name
        ));
    }

    let mut manager_to_team: HashMap<String, Vec<&Person>> = HashMap::new();
    for member in &team_members {
        manager_to_team
            .entry(member.manager.clone())
            .or_insert_with(Vec::new)
            .push(member);
    }

    for (manager_name, team) in manager_to_team {
        let mut team_record_label = String::from("{");
        let mut first_member = true;
        for member in team {
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
        team_record_label.push('}');
        let team_node_name = manager_name.replace(' ', "_");
        graphviz_code.push_str(&format!(
            "\"{}\" [shape=record, style=filled, fillcolor=orange,  label=\"{}\"];\n",
            team_node_name, team_record_label
        ));
        graphviz_code.push_str(&format!(
            "\"{}\" -> \"{}\";\n",
            manager_name, team_node_name
        ));
    }

    graphviz_code.push_str("}\n");
    graphviz_code
}
