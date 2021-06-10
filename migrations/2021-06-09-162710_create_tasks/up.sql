CREATE TABLE tasks
(
    id      BIGINT AUTO_INCREMENT NOT NULL PRIMARY KEY,
    content TEXT                  NOT NULL,
    done    BOOLEAN               NOT NULL DEFAULT 0
)
