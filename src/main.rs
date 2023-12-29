mod core;
use core::r#type::Topology;
use std::rc::Rc;

fn main() {
    let mut topology = Topology::new();

    // Creating 240 Access Switches with 48 interfaces each
    for i in 0..240 {
        let nic_names: Vec<String> = (1..=48).map(|n| format!("eth{}", n)).collect();
        topology.add_device(format!("Access{}", i), nic_names);
    }

    // Creating 12 Distribution Switches with 48 interfaces each
    for i in 0..12 {
        let nic_names: Vec<String> = (1..=48).map(|n| format!("eth{}", n)).collect();
        topology.add_device(format!("Distribution{}", i), nic_names);
    }

    // Creating 2 Core Switches with 48 interfaces each
    for i in 0..2 {
        let nic_names: Vec<String> = (1..=48).map(|n| format!("eth{}", n)).collect();
        topology.add_device(format!("Core{}", i), nic_names);
    }

    // Connect Access Switches to Distribution Switches
    for i in 0..240 {
        let access_switch_eth41 = Rc::clone(&topology.devices[i].borrow().nics[40]); // eth41 is at index 40
        let distribution_index = (i / 40) * 2;

        let distribution_switch1_eth =
            Rc::clone(&topology.devices[240 + distribution_index].borrow().nics[i % 40]); // eth1, eth2, ... for each distribution
        let distribution_switch2_eth =
            Rc::clone(&topology.devices[240 + distribution_index + 1].borrow().nics[i % 40]);

        topology.connect(access_switch_eth41.clone(), distribution_switch1_eth);
        topology.connect(access_switch_eth41, distribution_switch2_eth);
    }

    // Connect Distribution Switches to Core Switches
    for i in 0..12 {
        let distribution_switch_eth = Rc::clone(&topology.devices[240 + i].borrow().nics[40 + i]); // Using interfaces eth41, eth42, ...

        // Connect to Core Switch0
        let core_switch0_eth = Rc::clone(&topology.devices[252].borrow().nics[i]); // Using interfaces eth1 to eth12
        topology.connect(distribution_switch_eth.clone(), core_switch0_eth);

        // Connect to Core Switch1
        let core_switch1_eth = Rc::clone(&topology.devices[253].borrow().nics[i]); // Using interfaces eth1 to eth12
        topology.connect(distribution_switch_eth, core_switch1_eth);
    }
}
