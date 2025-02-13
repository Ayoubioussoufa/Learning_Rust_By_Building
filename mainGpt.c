#include <errno.h>
#include <string.h>
#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>
#include <netdb.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <fcntl.h>
#include <sys/select.h>

#define MAX_CLIENTS 100
#define BUFFER_SIZE 1024

void error(const char *msg) {
    write(2, msg, strlen(msg));
    write(2, "\n", 1);
    exit(1);
}

int extract_message(char **buf, char **msg) {
    char *newbuf;
    int i;

    *msg = 0;
    if (*buf == 0)
        return 0;
    i = 0;
    while ((*buf)[i]) {
        if ((*buf)[i] == '\n') {
            newbuf = calloc(1, sizeof(*newbuf) * (strlen(*buf + i + 1) + 1));
            if (newbuf == 0)
                return -1;
            strcpy(newbuf, *buf + i + 1);
            *msg = *buf;
            (*msg)[i + 1] = 0;
            *buf = newbuf;
            return 1;
        }
        i++;
    }
    return 0;
}

char *str_join(char *buf, char *add) {
    char *newbuf;
    int len;

    if (buf == 0)
        len = 0;
    else
        len = strlen(buf);
    newbuf = malloc(sizeof(*newbuf) * (len + strlen(add) + 1));
    if (newbuf == 0)
        return 0;
    newbuf[0] = 0;
    if (buf != 0)
        strcat(newbuf, buf);
    free(buf);
    strcat(newbuf, add);
    return newbuf;
}

int main(int argc, char *argv[]) {
    if (argc != 2) {
        error("Wrong number of arguments");
    }

    int sockfd, connfd, max_fd, client_fds[MAX_CLIENTS], client_count = 0;
    struct sockaddr_in servaddr, cli;
    fd_set read_fds;
    char buffer[BUFFER_SIZE];
    unsigned int len;

    memset(client_fds, -1, sizeof(client_fds));

    sockfd = socket(AF_INET, SOCK_STREAM, 0);
    if (sockfd == -1) {
        error("Fatal error");
    }

    // fcntl(sockfd, F_SETFL, O_NONBLOCK);

    bzero(&servaddr, sizeof(servaddr));
    servaddr.sin_family = AF_INET;
    servaddr.sin_addr.s_addr = htonl(INADDR_LOOPBACK); // 127.0.0.1
    servaddr.sin_port = htons(atoi(argv[1]));

    if (bind(sockfd, (struct sockaddr *)&servaddr, sizeof(servaddr)) != 0) {
        error("Fatal error");
    }

    if (listen(sockfd, 10) != 0) {
        error("Fatal error");
    }

    max_fd = sockfd;

    while (1) {
        FD_ZERO(&read_fds);
        FD_SET(sockfd, &read_fds);
        for (int i = 0; i < MAX_CLIENTS; i++) {
            if (client_fds[i] != -1) {
                FD_SET(client_fds[i], &read_fds);
                if (client_fds[i] > max_fd) {
                    max_fd = client_fds[i];
                }
            }
        }

        if (select(max_fd + 1, &read_fds, NULL, NULL, NULL) == -1) {
            error("Fatal error");
        }

        if (FD_ISSET(sockfd, &read_fds)) {
            len = sizeof(cli);
            connfd = accept(sockfd, (struct sockaddr *)&cli, &len);
            if (connfd < 0) {
                error("Fatal error");
            }
            // fcntl(connfd, F_SETFL, O_NONBLOCK);
            int client_id = client_count++;
            if (client_id >= MAX_CLIENTS) {
                close(connfd);
                continue;
            }
            client_fds[client_id] = connfd;
            sprintf(buffer, "server: client %d just arrived\n", client_id);
            for (int i = 0; i < client_count; i++) {
                if (client_fds[i] != -1 && client_fds[i] != connfd) {
                    send(client_fds[i], buffer, strlen(buffer), 0);
                }
            }
        }
        for (int i = 0; i < client_count; i++) {
            if (client_fds[i] != -1 && FD_ISSET(client_fds[i], &read_fds)) {
                int bytes_received = recv(client_fds[i], buffer, sizeof(buffer), 0);
                if (bytes_received <= 0) {
                    close(client_fds[i]);
                    sprintf(buffer, "server: client %d just left\n", i);
                    for (int j = 0; j < client_count; j++) {
                        if (client_fds[j] != -1 && client_fds[j] != client_fds[i]) {
                            send(client_fds[j], buffer, strlen(buffer), 0);
                        }
                    }
                    client_fds[i] = -1;
                } else {
                    buffer[bytes_received] = '\0';
                    for (int j = 0; j < client_count; j++) {
                        if (client_fds[j] != -1 && client_fds[j] != client_fds[i]) {
                            char msg[BUFFER_SIZE + 20];
                            sprintf(msg, "client %d: %s", i, buffer);
                            send(client_fds[j], msg, strlen(msg), 0);
                        }
                    }
                }
            }
        }
    }
    close(sockfd);
    return 0;
}
