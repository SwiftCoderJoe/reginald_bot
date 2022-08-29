module.exports = {
    apps : [{
      name: "reginaldbot-production",
      script: './reginald_bot',
      watch: false,
      time: true,
      env: {
        "DISCORD_TOKEN": ""
      }
    }],
  };
  