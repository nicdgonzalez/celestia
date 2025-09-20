package io.github.nicdgonzalez.celestia;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;

public class ServerStatusUpdate {
  private boolean isOnline;

  @JsonCreator
  public ServerStatusUpdate(boolean isOnline) {
    this.isOnline = isOnline;
  }

  @JsonProperty("is_online")
  public boolean isOnline() {
    return this.isOnline;
  }
}
