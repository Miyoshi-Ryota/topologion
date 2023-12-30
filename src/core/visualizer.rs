use super::r#type::Topology;


pub fn visualize_by_graphviz(topology: &Topology) -> String {
    let mut graph = String::from("graph template {\n");
    for device in &topology.devices {
        graph.push_str(&format!("    {} [\n        shape=oval\n        label={}\n    ];\n", device.borrow().name, device.borrow().name));
    }

    for link in &topology.links {
        let nic1 = &link.nic1;
        let nic2 = &link.nic2;
        let device1 = nic1.device.upgrade().unwrap();
        let device2 = nic2.device.upgrade().unwrap();
        graph.push_str(&format!("    {} -- {};\n", device1.borrow().name, device2.borrow().name));
    }

    return graph + "}";
}
