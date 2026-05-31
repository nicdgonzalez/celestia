# Sets the base image to use for subsequent instructions
FROM rockylinux/rockylinux:9.2

# Set the current working directory
WORKDIR /app

# Install any needed packages specified in requirements.txt
RUN dnf install -y tmux python python-pip java-17-openjdk java-17-openjdk-devel

# Copy the requirements.txt file from your host to your current location
COPY requirements*.txt ./

# Install any needed packages specified in requirements.txt
RUN pip install --no-cache-dir -r requirements.txt

# Make port 25565 available to the world outside this container
EXPOSE 25565

# ============================================================================
# Anything that is likely to change during development should be added last
# ============================================================================

# Copy the current directory contents into the container at {WORKDIR}
COPY . .

CMD ["./fuji", "start"]
