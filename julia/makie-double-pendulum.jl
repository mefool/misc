# pedulum - quite heavily borrowed from makie tutorial

# packages              # todo: brief description on why/how they are used
using DynamicalSystems
using OrdinaryDiffEq
using GLMakie
using DataStructures: CircularBuffer

## 1. Simulation Initialization
const L1 = 1.0
const L2 = 0.9
M = 2
u0 = [π/3, 0, 3π/4, -2]
dp = Systems.double_pendulum(u0; L1, L2)

# Solver for the differential equations, with constant step
diffeq = (alg = Tsit5(), adaptive = false, dt = 0.005)
integ = integrator(dp, u0; diffeq)

function xycoords(state)
    θ1 = state[1]
    θ2 = state[3]
    x1 = L1 * sin(θ1)
    y1 = -L1 * cos(θ1)
    x2 = x1 + L2 * sin(θ2)
    y2 = y1 - L2 * cos(θ2)
    return x1,x2,y1,y2
end

function progress_for_one_step!(integ)
    step!(integ) # progressing the integrator
    u = integ.u
    return xycoords(u)
end

## 2. Observables of the simulation
x1,x2,y1,y2 = xycoords(u0)
rod   = Observable([Point2f(0, 0), Point2f(x1, y1), Point2f(x2, y2)])
balls = Observable([Point2f(x1, y1), Point2f(x2, y2)])

tail = 300
traj = CircularBuffer{Point2f}(tail)
fill!(traj, Point2f(x2, y2))
traj = Observable(traj)

## 3. Plot the Obeservables and static elements
# Initializing the figure
fig = Figure(); display(fig)
ax = Axis(fig[1,1])

# Now we plot the observables _directly_! First the pendulum
lines!(ax, rod; linewidth = 4, color = :purple)
scatter!(ax, balls; marker = :circle, strokewidth = 2,
    strokecolor = :purple,
    color = :black, markersize = [8, 12]
)

# then its trajectory, with a nice fadeout color
c = to_color(:purple)
tailcol = [RGBAf(c.r, c.g, c.b, (i/tail)^2) for i in 1:tail]
lines!(ax, traj; linewidth = 3, color = tailcol)

# We can also plot now any other static elements
ax.title = "double pendulum"
ax.aspect = DataAspect()
l = 1.05(L1+L2)
xlims!(ax, -l, l)
ylims!(ax, -l, 0.5l)

## 4. Animation step function
function animstep!(integ, rod, balls, traj)
    x1,x2,y1,y2 = progress_for_one_step!(integ)
    rod[] = [Point2f(0, 0), Point2f(x1, y1), Point2f(x2, y2)]
    balls[] = [Point2f(x1, y1), Point2f(x2, y2)]
    push!(traj[], Point2f(x2, y2))
    traj[] = traj[] # <- important! Updating in-place the value of an
                    # `Observable` does not trigger an update!
end

## 5. Test
# 5.1 unwrapped
if false
    for i in 1:1000
        animstep!(integ, rod, balls, traj)
        sleep(0.001)
    end
end

# 5.2 wrapped
function makefig(u0)
    dp = Systems.double_pendulum(u0; L1, L2)
    integ = integrator(dp, u0; diffeq)
    x1,x2,y1,y2 = xycoords(u0)
    rod   = Observable([Point2f(0, 0), Point2f(x1, y1), Point2f(x2, y2)])
    balls = Observable([Point2f(x1, y1), Point2f(x2, y2)])
    traj = CircularBuffer{Point2f}(tail)
    fill!(traj, Point2f(x2, y2)) # add correct values to the circular buffer
    traj = Observable(traj) # make it an observable
    fig = Figure(); display(fig)
    ax = Axis(fig[1,1])
    lines!(ax, rod; linewidth = 4, color = :purple)
    scatter!(ax, balls; marker = :circle, strokewidth = 2,
        strokecolor = :purple,
        color = :black, markersize = [8, 12]
    )
    lines!(ax, traj; linewidth = 3, color = tailcol)
    ax.title = "double pendulum"
    ax.aspect = DataAspect()
    l = 1.05(L1+L2)
    xlims!(ax, -l, l)
    ylims!(ax, -l, l)
    # also return the figure object, we'll ues it!
    return fig, integ, rod, balls, traj
end

## 6. Saving to videos
if false
    fig, integ, rod, balls, traj = makefig(u0)
    frames = 1:200
    record(fig, "video.mp4", frames; framerate = 60) do i # i = frame number
        for j in 1:5 # step 5 times per frame
            animstep!(integ, rod, balls, traj)
        end
    end # for each step of this loop, a frame is recorded
end

## 7. Interactive application
fig, integ, rod, balls, traj = makefig(u0)
# 7.1 Add a run button
run = Button(fig[2,1][1,1][1,1]; label = "run", width=100)
isrunning = Observable(false)
on(run.clicks) do clicks; isrunning[] = !isrunning[]; end
on(run.clicks) do clicks
    @async while isrunning[]
        isopen(fig.scene) || break # ensures computations stop if closed window
        animstep!(integ, rod, balls, traj)
        sleep(0.001) # or `yield()` instead
    end
end

# 7.2 new initial condiction when sliders do things
sg = SliderGrid(
    fig[2, 1][1,2],
    (label = "θ1", range = -π: 0.01: +π, format = "{:.2f}", startvalue = π/3),
    (label = "ω1", range = -5: 0.01: +5, format = "{:.2f}", startvalue = 0),
    (label = "θ2", range = -π: 0.01: +π, format = "{:.2f}", startvalue = 3π/4),
    (label = "ω2", range = -5: 0.01: +5, format = "{:.2f}", startvalue = -2),
    tellheight = true
)

sliderobservables = [s.value for s in sg.sliders]

u_dict = ["θ1", "ω1", "θ2", "ω2"]
u_vec = [π/3, 0, 3π/4, -2]
for i in 1:length(sg.sliders)
    on(sg.sliders[i].value) do slval
        u_vec[i] = slval
        # println(u_dict[i],": ", slval)
    end
end

# 7.3 Add a update button
update = Button(fig[2,1][1,1][2,1]; label = "update", width=100)

on(update.clicks) do clicks
    u = u_vec
    reinit!(integ, u)
    # Reset tail and balls to new coordinates
    x1,x2,y1,y2 = xycoords(u)
    traj[] .= fill(Point2f(x2, y2), length(traj[]))
    traj[] = traj[]
    rod[] = [Point2f(0, 0), Point2f(x1, y1), Point2f(x2, y2)]
    balls[] = [Point2f(x1, y1), Point2f(x2, y2)]
end
