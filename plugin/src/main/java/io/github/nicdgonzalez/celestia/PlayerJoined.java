package io.github.nicdgonzalez.celestia;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.UUID;

public class PlayerJoined {
  private UUID uuid;
  private String username;

  @JsonCreator
  public PlayerJoined(@JsonProperty("uuid") UUID uuid, @JsonProperty("username") String username) {
    this.uuid = uuid;
    this.username = username;
  }

  public UUID getUuid() {
    return this.uuid;
  }

  public String getUsername() {
    return this.username;
  }
}
