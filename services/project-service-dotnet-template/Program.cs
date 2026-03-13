using Google.Protobuf.WellKnownTypes;
using Grpc.Core;
using Microsoft.AspNetCore.Server.Kestrel.Core;
using OpenTelemetry.Resources;
using OpenTelemetry.Trace;
using ProjectProto = Project;

var builder = WebApplication.CreateBuilder(args);

builder.WebHost.ConfigureKestrel(options =>
{
    options.ListenAnyIP(50054, listenOptions =>
    {
        listenOptions.Protocols = HttpProtocols.Http2;
    });
});

builder.Services.AddGrpc();
builder.Services.AddOpenTelemetry()
    .ConfigureResource(resource => resource.AddService("project-service"))
    .WithTracing(tracing =>
    {
        tracing.AddAspNetCoreInstrumentation();
        tracing.AddOtlpExporter();
    });

var app = builder.Build();
app.MapGrpcService<ProjectServiceImpl>();
app.MapGet("/", () => "Use a gRPC client to communicate with this service.");
app.Run();

public class ProjectServiceImpl : ProjectProto.ProjectService.ProjectServiceBase
{
    private readonly List<ProjectProto.Project> _projects =
    [
        new ProjectProto.Project
        {
            Id = "project-1",
            Title = "Telemetry Dashboard",
            Description = "Build a dashboard for distributed tracing.",
            TeacherId = "teacher-1",
            MaxStudentsPerTeam = 3,
            StartDate = Timestamp.FromDateTime(DateTime.UtcNow),
            EndDate = Timestamp.FromDateTime(DateTime.UtcNow.AddMonths(3)),
            SubjectId = "subject-2",
        },
        new ProjectProto.Project
        {
            Id = "project-2",
            Title = "Secure Boot Research",
            Description = "Investigate secure boot flows for embedded devices.",
            TeacherId = "teacher-2",
            MaxStudentsPerTeam = 2,
            StartDate = Timestamp.FromDateTime(DateTime.UtcNow),
            EndDate = Timestamp.FromDateTime(DateTime.UtcNow.AddMonths(4)),
            SubjectId = "subject-1",
        },
    ];

    private readonly Dictionary<string, ProjectProto.Team> _teams = new();

    public override Task<ProjectProto.ListProjectsResponse> ListProjects(
        ProjectProto.ListProjectsRequest request,
        ServerCallContext context)
    {
        var response = new ProjectProto.ListProjectsResponse();
        response.Projects.AddRange(_projects);
        return Task.FromResult(response);
    }

    public override Task<ProjectProto.Project> GetProject(
        ProjectProto.GetProjectRequest request,
        ServerCallContext context)
    {
        var project = _projects.FirstOrDefault(project => project.Id == request.ProjectId);
        if (project is null)
        {
            throw new RpcException(new Status(StatusCode.NotFound, "project not found"));
        }

        return Task.FromResult(project);
    }

    public override Task<ProjectProto.Team> RegisterTeam(
        ProjectProto.RegisterTeamRequest request,
        ServerCallContext context)
    {
        if (_projects.All(project => project.Id != request.ProjectId))
        {
            throw new RpcException(new Status(StatusCode.NotFound, "project not found"));
        }

        var team = new ProjectProto.Team
        {
            Id = $"team-{_teams.Count + 1}",
            ProjectId = request.ProjectId,
            Name = $"Team {_teams.Count + 1}",
            LeaderStudentId = request.CreatorStudentId,
        };
        team.StudentIds.Add(request.CreatorStudentId);
        _teams[team.Id] = team;
        return Task.FromResult(team);
    }

    public override Task<ProjectProto.Team> AddTeamMember(
        ProjectProto.AddTeamMemberRequest request,
        ServerCallContext context)
    {
        if (!_teams.TryGetValue(request.TeamId, out var team))
        {
            throw new RpcException(new Status(StatusCode.NotFound, "team not found"));
        }

        if (!team.StudentIds.Contains(request.StudentId))
        {
            team.StudentIds.Add(request.StudentId);
        }

        return Task.FromResult(team);
    }

    public override Task<ProjectProto.Team> RemoveTeamMember(
        ProjectProto.RemoveTeamMemberRequest request,
        ServerCallContext context)
    {
        if (!_teams.TryGetValue(request.TeamId, out var team))
        {
            throw new RpcException(new Status(StatusCode.NotFound, "team not found"));
        }

        team.StudentIds.Remove(request.StudentId);
        return Task.FromResult(team);
    }
}
