# clone all the docker dependencies with svn
# ------------------------------------------
# container.go
svn export https://github.com/containerd/containerd/cio
svn export https://github.com/docker/docker/api/types/container
svn export https://github.com/docker/docker/api/types/mount
svn export https://github.com/docker/docker/api/types/swarm
svn export https://github.com/docker/docker/container/stream
svn export https://github.com/docker/docker/daemon/exec
svn export https://github.com/docker/docker/daemon/logger
svn export https://github.com/docker/docker/daemon/logger/jsonfilelog
svn export https://github.com/docker/docker/daemon/logger/local
svn export https://github.com/docker/docker/daemon/network
svn export https://github.com/docker/docker/errdefs
