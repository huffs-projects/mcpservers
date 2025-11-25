use crate::models::WaybarTemplate;

pub fn query_templates(use_case: Option<String>) -> Vec<WaybarTemplate> {
    let mut templates = Vec::new();

    if let Some(ref case) = use_case {
        match case.as_str() {
            "hyprland-default" => {
                templates.push(create_hyprland_template());
            }
            "battery" => {
                templates.push(create_battery_template());
            }
            "network" => {
                templates.push(create_network_template());
            }
            "cpu" => {
                templates.push(create_cpu_template());
            }
            _ => {
                templates.extend(get_all_templates());
            }
        }
    } else {
        templates.extend(get_all_templates());
    }

    templates
}

fn get_all_templates() -> Vec<WaybarTemplate> {
    vec![
        create_hyprland_template(),
        create_battery_template(),
        create_network_template(),
        create_cpu_template(),
    ]
}

fn create_hyprland_template() -> WaybarTemplate {
    WaybarTemplate::new(
        "hyprland-default".to_string(),
        r#"{
  "layer": "top",
  "position": "top",
  "height": 30,
  "spacing": 4,
  "modules-left": ["hyprland/workspaces"],
  "modules-center": ["clock"],
  "modules-right": ["idle_inhibitor", "pulseaudio", "network", "cpu", "memory", "battery", "tray"],
  "hyprland/workspaces": {
    "disable-scroll": true,
    "format": "{name}: {icon}",
    "format-icons": {
      "1": "󰨞",
      "2": "󰈹",
      "3": "󰆍",
      "4": "󰊴",
      "5": "󰞷"
    }
  },
  "clock": {
    "format": "{:%Y-%m-%d %H:%M}",
    "format-alt": "{:%A %B %d, %Y}"
  },
  "idle_inhibitor": {
    "format": "{icon}",
    "format-icons": {
      "activated": "󰒳",
      "deactivated": "󰒲"
    }
  },
  "pulseaudio": {
    "format": "{volume}% {icon}",
    "format-muted": "󰝟 Muted",
    "format-icons": {
      "headphone": "󰋋",
      "hands-free": "󰋎",
      "headset": "󰋎",
      "phone": "󰄜",
      "portable": "󰦧",
      "car": "󰄋",
      "default": ["󰕿", "󰖀", "󰕾"]
    }
  },
  "network": {
    "format-wifi": "󰤨 {essid}",
    "format-ethernet": "󰈀 {ifname}",
    "format-disconnected": "󰤭 Disconnected"
  },
  "cpu": {
    "format": "󰻠 {usage}%",
    "interval": 2
  },
  "memory": {
    "format": "󰍛 {}%",
    "interval": 2
  },
  "battery": {
    "states": {
      "warning": 30,
      "critical": 15
    },
    "format": "{capacity}% {icon}",
    "format-charging": "{capacity}% 󰂄",
    "format-plugged": "{capacity}% 󰂄",
    "format-alt": "{time} {icon}",
    "format-icons": ["󰁺", "󰁻", "󰁼", "󰁽", "󰁾", "󰁿", "󰂀", "󰂁", "󰂂", "󰁹"]
  },
  "tray": {
    "spacing": 10
  }
}"#.to_string(),
        "Default Hyprland configuration with workspaces, clock, and system modules".to_string(),
    )
    .with_css(r#"* {
  border: none;
  border-radius: 0;
  font-family: "JetBrainsMono Nerd Font";
  font-size: 13px;
  min-height: 0;
}

window#waybar {
  background-color: #1e1e2e;
  color: #cdd6f4;
}

#hyprland-workspaces button {
  padding: 0 5px;
  background-color: transparent;
  color: #cdd6f4;
}

#hyprland-workspaces button.focused {
  background-color: #89b4fa;
  color: #1e1e2e;
}

#clock, #idle_inhibitor, #pulseaudio, #network, #cpu, #memory, #battery, #tray {
  padding: 0 10px;
  margin: 0 4px;
}

tooltip {
  background-color: #1e1e2e;
  color: #cdd6f4;
  border-radius: 5px;
}"#.to_string())
    .with_modules(vec![
        "hyprland/workspaces".to_string(),
        "clock".to_string(),
        "idle_inhibitor".to_string(),
        "pulseaudio".to_string(),
        "network".to_string(),
        "cpu".to_string(),
        "memory".to_string(),
        "battery".to_string(),
        "tray".to_string(),
    ])
    .with_style_selectors(vec![
        "window#waybar".to_string(),
        "#hyprland-workspaces".to_string(),
        "#clock".to_string(),
        "#idle_inhibitor".to_string(),
        "#pulseaudio".to_string(),
        "#network".to_string(),
        "#cpu".to_string(),
        "#memory".to_string(),
        "#battery".to_string(),
        "#tray".to_string(),
        "tooltip".to_string(),
    ])
}

fn create_battery_template() -> WaybarTemplate {
    WaybarTemplate::new(
        "battery".to_string(),
        r#"{
  "battery": {
    "bat": "BAT0",
    "interval": 60,
    "states": {
      "warning": 30,
      "critical": 15
    },
    "format": "{capacity}% {icon}",
    "format-charging": "{capacity}% 󰂄",
    "format-plugged": "{capacity}% 󰂄",
    "format-alt": "{time} {icon}",
    "format-icons": ["󰁺", "󰁻", "󰁼", "󰁽", "󰁾", "󰁿", "󰂀", "󰂁", "󰂂", "󰁹"]
  }
}"#.to_string(),
        "Battery module configuration with charging states and icons".to_string(),
    )
    .with_css(r#"#battery {
  padding: 0 10px;
  margin: 0 4px;
}

#battery.warning {
  color: #f9e2af;
}

#battery.critical {
  color: #f38ba8;
}

#battery.charging {
  color: #a6e3a1;
}"#.to_string())
    .with_modules(vec!["battery".to_string()])
    .with_style_selectors(vec!["#battery".to_string()])
}

fn create_network_template() -> WaybarTemplate {
    WaybarTemplate::new(
        "network".to_string(),
        r#"{
  "network": {
    "format-wifi": "󰤨 {essid} ({signalStrength}%)",
    "format-ethernet": "󰈀 {ifname}: {ipaddr}",
    "format-disconnected": "󰤭 Disconnected",
    "format-linked": "{ifname} (No IP)",
    "interval": 5
  }
}"#.to_string(),
        "Network module with WiFi and Ethernet support".to_string(),
    )
    .with_css(r#"#network {
  padding: 0 10px;
  margin: 0 4px;
}

#network.disconnected {
  color: #f38ba8;
}

#network.ethernet {
  color: #89b4fa;
}

#network.wifi {
  color: #a6e3a1;
}"#.to_string())
    .with_modules(vec!["network".to_string()])
    .with_style_selectors(vec!["#network".to_string()])
}

fn create_cpu_template() -> WaybarTemplate {
    WaybarTemplate::new(
        "cpu".to_string(),
        r#"{
  "cpu": {
    "format": "󰻠 {usage}%",
    "interval": 2
  }
}"#.to_string(),
        "CPU usage monitoring module".to_string(),
    )
    .with_css(r#"#cpu {
  padding: 0 10px;
  margin: 0 4px;
}

#cpu.high {
  color: #f9e2af;
}

#cpu.critical {
  color: #f38ba8;
}"#.to_string())
    .with_modules(vec!["cpu".to_string()])
    .with_style_selectors(vec!["#cpu".to_string()])
}

