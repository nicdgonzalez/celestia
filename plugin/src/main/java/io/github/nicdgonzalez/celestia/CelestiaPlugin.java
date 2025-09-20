package io.github.nicdgonzalez.celestia;

import org.bukkit.event.EventHandler;
import org.bukkit.event.Listener;
import org.bukkit.event.player.PlayerJoinEvent;
import org.bukkit.event.server.ServerLoadEvent;
import org.bukkit.plugin.java.JavaPlugin;

import com.destroystokyo.paper.event.player.PlayerConnectionCloseEvent;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.PropertyNamingStrategies;

import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.net.http.HttpRequest.BodyPublishers;
import java.util.UUID;

import org.bukkit.Bukkit;
import org.bukkit.entity.Player;

public class CelestiaPlugin extends JavaPlugin implements Listener {
  private HttpClient client = HttpClient.newHttpClient();

  @Override
  public void onEnable() {
    Bukkit.getPluginManager().registerEvents(this, this);
  }

  @Override
  public void onDisable() {
    ServerStatusUpdate value = new ServerStatusUpdate(false);
    ObjectMapper mapper = this.getMapper();
    String body;

    try {
      body = mapper.writeValueAsString(value);
    } catch (Exception e) {
      this.getLogger().info(e.toString());
      e.printStackTrace();
      return;
    }

    this.sendToBackend("status", body);
  }

  @EventHandler
  public void onServerLoad(ServerLoadEvent event) {
    ServerStatusUpdate value = new ServerStatusUpdate(true);
    ObjectMapper mapper = this.getMapper();
    String body;

    try {
      body = mapper.writeValueAsString(value);
    } catch (Exception e) {
      this.getLogger().info(e.toString());
      e.printStackTrace();
      return;
    }

    this.sendToBackend("status", body);
  }

  @EventHandler
  public void onPlayerJoin(PlayerJoinEvent event) {
    Player player = event.getPlayer();
    UUID uuid = player.getUniqueId();
    String username = player.getName();
    PlayerJoined playerJoinedData = new PlayerJoined(uuid, username);

    ObjectMapper mapper = this.getMapper();
    String body;

    try {
      body = mapper.writeValueAsString(playerJoinedData);
    } catch (Exception e) {
      this.getLogger().info(e.toString());
      e.printStackTrace();
      return;
    }

    this.sendToBackend("player_joined", body);
  }

  @EventHandler
  public void onPlayerDisconnect(PlayerConnectionCloseEvent event) {
    UUID uuid = event.getPlayerUniqueId();
    String username = event.getPlayerName();
    PlayerJoined playerJoinedData = new PlayerJoined(uuid, username);

    ObjectMapper mapper = this.getMapper();
    String body;

    try {
      body = mapper.writeValueAsString(playerJoinedData);
    } catch (Exception e) {
      this.getLogger().info(e.toString());
      e.printStackTrace();
      return;
    }

    this.sendToBackend("player_left", body);
  }

  private ObjectMapper getMapper() {
    ObjectMapper mapper = new ObjectMapper();
    mapper.setPropertyNamingStrategy(PropertyNamingStrategies.SNAKE_CASE);
    return mapper;
  }

  private void sendToBackend(String callback, String body) {
    String url = String.format("http://127.0.0.1:1140/callback/%s", callback);
    this.getLogger().info(String.format("Sending POST request to %s: %s", url, body));

    HttpRequest request = HttpRequest.newBuilder()
        .uri(URI.create(url))
        .header("Content-Type", "application/json")
        .POST(BodyPublishers.ofString(body))
        .build();

    this.client.sendAsync(request, HttpResponse.BodyHandlers.ofString())
        .thenApply(HttpResponse::body)
        .join();
  }
}
