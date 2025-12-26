# Tracing and Telemetry Guidelines

## Overview

This document outlines best practices, antipatterns, and recommendations for
tracing and telemetry in the Empire project. Consistent and effective tracing
is essential for monitoring, debugging, and understanding the behavior of our
application in production.

## Current Implementation Analysis

The current tracing implementation in the Empire project has several strengths and weaknesses:

### Strengths

- Use of the `tracing` ecosystem for structured logging
- Configuration of both console and file outputs
- Log file rotation with size limits
- Use of span-based tracing with the `#[instrument]` macro in some areas
- The appropriate use of different log levels in some components

### Weaknesses

- Inconsistent use of log levels across different modules
- Inconsistent use of the `#[instrument]` macro
- Missing instrumentation in critical paths (especially authentication)
- Inconsistent error logging practices
- Some overly verbose debug logs
- No standardized format for log messages

## Log Levels

Use the following log levels consistently throughout the codebase:

| Level   | Purpose                                          | Examples                                                      |
| ------- | ------------------------------------------------ | ------------------------------------------------------------- |
| `error` | Critical issues that require immediate attention | Database connection failures, API failures, security breaches |
| `warn`  | Non-critical issues that might need attention    | Failed operations that can be retried, deprecated API usage   |
| `info`  | Important operational events                     | Server startup, user login/logout, significant state changes  |
| `debug` | Detailed operational information                 | Function entry/exit with key parameters, state transitions    |
| `trace` | Very detailed debugging information              | Full request/response payloads, detailed algorithm steps      |

## Best Practices

1. **Use the `#[instrument]` macro for all public methods**
   - Skip large parameters with `#[instrument(skip(param1, param2))]`
   - Add fields for context with `#[instrument(fields(user_id = ?user.id))]`

2. **Log at appropriate levels**
   - Use `info` for events that operators need to see in normal operation
   - Use `debug` for information useful during development and troubleshooting
   - Use `trace` for very detailed information needed only for deep debugging

3. **Include context in log messages**
   - Always include relevant IDs (user ID, session ID, job ID, etc.)
   - Format complex objects with debug formatting (`{:?}`) only at `trace` level

4. **Log both entry and exit of critical operations**
   - Log at `debug` level when entering a critical operation
   - Log at `info` level when completing a critical operation
   - Log at `warn` or `error` level when an operation fails

5. **Use structured logging**
   - Use the `span!` and `event!` macros for complex logging scenarios
   - Add fields to spans for additional context

6. **Log all errors**
   - Log at `error` level for unexpected errors
   - Log at `warn` level for expected errors (e.g., validation failures)
   - Include error details and context

7. **Skip instrumenting `new` and `drop` methods**
   - These methods are called frequently and can cause performance issues

## Anti-Patterns

1. **Logging sensitive information**
   - Never log passwords, tokens, or other sensitive data
   - Use `#[instrument(skip(password))]` to exclude sensitive parameters

2. **Excessive logging**
   - Avoid logging entire objects at `debug` level
   - Don't log in tight loops without level checks

3. **Inconsistent log levels**
   - Don't use `info` for detailed debugging information
   - Don't use `debug` for critical operational events

4. **Missing context**
   - Avoid generic log messages without context (e.g., "Operation failed")
   - Always include relevant IDs and error details

5. **Ignoring errors in logging**
   - Don't swallow errors without logging them
   - Don't log errors without context

## Standardized Log Message Formats

Use the following formats for consistency:

1. **Operation Start**

   ```
   Starting [operation] for [entity] [ID]
   ```

2. **Operation Success**

   ```
   Completed [operation] for [entity] [ID] in [duration]
   ```

3. **Operation Failure**

   ```
   Failed to [operation] for [entity] [ID]: [error]
   ```

4. **State Change**

   ```
   [Entity] [ID] state changed from [old_state] to [new_state]
   ```

5. **Resource Usage**
   ```
   [Resource] usage: [current]/[limit] ([percentage]%)
   ```

## Recommendations for Improvement

1. **Standardize tracing across all modules**
   - Add the `#[instrument]` macro to all public methods
   - Ensure consistent use of log levels

2. **Enhance authentication and session tracing**
   - Add comprehensive tracing to the `auth` module
   - Log all session creation, validation, and invalidation events

3. **Improve error logging**
   - Ensure all error paths include appropriate logging
   - Add context to error logs

4. **Reduce verbosity of debug logs**
   - Move detailed object dumps to trace level
   - Format complex objects more concisely

5. **Add metrics collection**
   - Track operation durations
   - Monitor resource usage
   - Count error occurrences

6. **Enhance telemetry configuration**
   - Add support for structured JSON logging
   - Configure different log levels for different environments
   - Add correlation IDs for request tracing

## Examples

### Good Examples

```rust
#[instrument(skip(password))]
pub fn authenticate_user(username: &str, password: &str) -> Result<User> {
    debug!("Authenticating user {}", username);
    // Authentication logic
    if authenticated {
        info!("User {} successfully authenticated", username);
        Ok(user)
    } else {
        warn!("Failed authentication attempt for user {}", username);
        Err(Error::AuthenticationFailed)
    }
}

#[instrument(skip(queue))]
async fn process_job(queue: &JobQueue, job: Job) -> Result<()> {
    debug!("Processing job {}", job.id);
    let start = Instant::now();
    let result = perform_job_logic(job).await;
    let duration = start.elapsed();

    match result {
        Ok(_) => {
            info!("Completed job {} in {:?}", job.id, duration);
            Ok(())
        }
        Err(e) => {
            error!("Failed to process job {}: {}", job.id, e);
            Err(e)
        }
    }
}
```

### Bad Examples

```rust
// Too verbose at debug level
debug!("User object: {:?}", user);  // Should be trace!

// Missing context
error!("Operation failed");  // Should include what operation and why

// Wrong log level
info!("Database query result: {:?}", results);  // Should be debug or trace

// Missing error logging
fn process() -> Result<()> {
    if let Err(e) = some_operation() {
        return Err(e);  // Error not logged before propagating
    }
    Ok(())
}
```

## Conclusion

Consistent and effective tracing is essential for maintaining and troubleshooting
the Empire application. By following these guidelines, we can improve the
observability of our system and make it easier to identify and resolve issues in
production.
