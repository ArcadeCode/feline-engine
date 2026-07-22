# Job scheduler (Manager) of Feline Engine

> [!WARNING] Need rework
> This section show too much for an simple concept file, code don't need to be this specific, need to rework on that.

The job scheduler is a module of Feline Engine who manage the execution of jobs in parallel, with a focus on ECS updates. The job scheduler can send jobs to CPU threads to be executed in parallel and concurency.

> [!NOTE] In a future-proof the Job scheduler will be send GPU jobs.

## Communication with the Job scheduler
The Engine communicate with the Job scheduler by using the [EDA System](../eda/main.md)

The Job scheduler use a different approach of the main bus, he contain a FIFO queue of jobs to send to the CPU, when we send an event `JobRequestEvent`, this event is listen by the Job scheduler and send in his FIFO queue with `JobRequestAcceptedEvent`, when a Job is send to the CPU, the scheduler send a `JobRequestInExecutionEvent` and when the CPU is done with the job : `JobRequestDone` with the result has content.

### Documentation
The Job scheduler subscribed event :
- `JobRequestEvent` : The Engine send this event to the Job scheduler to request a job to be executed, the content of this event is the job itself with all the data needed to execute it, such as the function pointer, the arguments, the priority, etc...
- `JobWorkloadRequestEvent` : The Engine send this event to the Job scheduler to request the current workload of the Job scheduler, the content of this event is empty, the Job scheduler will respond with a `JobWorkloadResponseEvent` with the current workload of the Job scheduler.

The Job scheduler pusblished event :
- `JobRequestAcceptedEvent` : The Job scheduler send this event to the Engine to inform that the job request has been accepted and is in the FIFO queue, the content of this event is the job itself with all the data needed to execute it, such as the function pointer, the arguments, the priority, etc...
- `JobRequestInExecutionEvent` : The Job scheduler send this event to the Engine to inform that the job is in execution, the content of this event is the job itself with all the data needed to execute it, such as the function pointer, the arguments, the priority, etc...
- `JobRequestDoneEvent` : The Job scheduler send this event to the Engine to inform that the job is done, the content of this event is the job itself with all the data needed to execute it, such as the function pointer, the arguments, the priority, etc... and the result of the job.
- `JobSchedulerOverpressuredEvent` : The Job scheduler send this event to the Engine to inform that the Job scheduler is overpressured, the content of this event is empty, the Engine will throtle some systems to lift up the charge on the Job scheduler.
- `JobWorkloadResponseEvent` : The Job scheduler send this event to the Engine to respond to a `JobWorkloadRequestEvent`, the content of this event is the current workload of the Job scheduler.
- `JobSchedulerDetail` : The Job scheduler send this event to the Engine to inform about the details of the Job scheduler, such as the number of threads, the number of jobs in the FIFO queue, etc...

> [!TIPS] FIFO queue
> The FIFO queue can be resized up to all the ram of the system but, if the Job scheduler start flooding under the tasks, it will send a `JobSchedulerOverpressuredEvent`, some systems will throtle in attempt of lift up the charge on the Job sheduler.

### `JobRequestEvent` format
A job request event contain this following variables :
```cpp
class JobRequestEvent extend Event {
    void (*jobFunction)(void*); // Pointer to the function to execute
    void* jobData; // Pointer to the data needed to execute the job
    bool forceMainThread; // If true (false by default), the job will be executed on the main thread, otherwise it can be executed on any thread available
    int priority; // Priority of the job, higher is more important
};
```

Simple example of multiplying all number of an 2x2 array by 2 in parallel with the Job scheduler :
```cpp
#include <FelineEngine/EDA/EDA.h>
EDA::getInstance();

void multiplyByTwo(void* data) {
    int* array = (int*)data;
    for (int i = 0; i < 4; i++) {
        array[i] *= 2;
    }
}

int main() {
    int array[4] = {1, 2, 3, 4};

    EDA::getInstance().subscribe<EngineReadyEvent>([](EngineReadyEvent event) {

    JobRequestEvent jobRequest;
    jobRequest.jobFunction = multiplyByTwo;
    jobRequest.jobData = array;
    jobRequest.forceMainThread = false;
    jobRequest.priority = 1;
    EDA::getInstance().publish(jobRequest);
    // Wait for the job to be done and get the result
    // ...
}
```

> [!WARNING] This format represent a **requested job** and not a job of the scheduler, Real job contain ID and other variables such as the state of the job, the time of execution, etc... This format is only used to request a job to the Job scheduler, the Job scheduler will then create a real job with this data and add it to his FIFO queue.

### `JobRequestDoneEvent` format
A job request done event contain this following variables :
```cpp
class JobRequestDoneEvent extend Event {
    void (*jobFunction)(void*); // Pointer to the function that has been executed
    void* jobData; // Pointer to the data that has been used to execute the job
    int priority; // Priority of the job, higher is more important
    void* result; // Pointer to the result of the job, can be null if the job doesn't return anything
};
```

Example of how to get the result of a job in the Engine :
```cpp
#include <FelineEngine/EDA/EDA.h>
EDA::getInstance();

EDA::getInstance().subscribe<JobRequestDoneEvent>([](JobRequestDoneEvent event) {
    if (event.jobFunction == multiplyByTwo) {
        int* result = (int*)event.result;
        // Do something with the result
    }
});
```

## Job sheduler implementation
???

## Future-proofing the Job scheduler
In a future-proofing perspective, the Job scheduler will be designed to be able to send jobs to the GPU by sending shader, this however is limited by GPU architecture and the fact that GPU are not designed to execute any kind of code. This will in a long term perspective resolve by the [PSCE engine](../psce/main.md) a Polymorphic Stencil Compute Engine who will be able to execute rules on a 2D grid in parallel on CPU or GPU, the Job scheduler will then send jobs to the PSCE engine to execute them on the GPU, this will allow us to execute more complex jobs on the GPU without having to worry about the limitations of GPU architecture.