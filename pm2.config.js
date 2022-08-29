module.exports = {
    apps : [{
      name: "reginaldbot-production",
      script: './reginald_bot',
      interpreter: "none",
      watch: false,
      time: true,
      env: {
        "DISCORD_TOKEN": ""
      }
    }],
  };
  