CREATE TYPE job_status AS ENUM ('pending', 'in_progress', 'completed', 'failed', 'cancelled');
CREATE TYPE job_type AS ENUM ('modifier', 'building', 'resource');

CREATE TABLE IF NOT EXISTS jobs
(
    id              UUID        NOT NULL DEFAULT generate_ulid(),
    job_type        job_type    NOT NULL,
    status          job_status  NOT NULL DEFAULT 'pending',
    payload         JSONB       NOT NULL,
    run_at          TIMESTAMPTZ NOT NULL,
    last_error      TEXT        NULL,
    retries         INTEGER     NOT NULL DEFAULT 0,
    max_retries     INTEGER     NOT NULL DEFAULT 3,
    priority        INTEGER     NOT NULL DEFAULT 50,
    timeout_seconds INTEGER     NOT NULL DEFAULT 60,
    locked_at       TIMESTAMPTZ NULL,
    locked_by       TEXT        NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),

    PRIMARY KEY (id)
);

CREATE INDEX idx_jobs_status_run_at ON jobs (status, run_at) WHERE status = 'pending';
CREATE INDEX idx_jobs_locked_by ON jobs (locked_by) WHERE locked_by IS NOT NULL;
