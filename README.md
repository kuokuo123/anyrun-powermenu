# Anyrun-Powermenu

A simple [anyrun](https://github.com/Kirottu/anyrun) plugin to shutdown computer or logout user session through systemd commands. The specific commands for each action can be customized through a config file, which allows non-systemd guys to use this plugin.

## Usage

Simple as it seems.

## Configuration

Default config

```ron
// <Anyrun config dir>/powermenu.ron

Config(

  prefix: "p ",

  engines: [

    Custom(
      name: "Lock",
      cmd: "swaylock",
      icon: "system-lock-screen",
    ),

    Logout,
    Suspend,
    Hibernate,
    Reboot,
    Shutdown,

  ],

  max_entries: 12,
)
```
