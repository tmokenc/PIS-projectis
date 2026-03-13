FROM mcr.microsoft.com/dotnet/sdk:9.0 AS build
WORKDIR /src

COPY proto /src/proto
COPY services/project-service-dotnet-template /src/services/project-service-dotnet-template

WORKDIR /src/services/project-service-dotnet-template
RUN dotnet restore
RUN dotnet publish -c Release -o /app/publish

FROM mcr.microsoft.com/dotnet/aspnet:9.0 AS runtime
WORKDIR /app
COPY --from=build /app/publish .

EXPOSE 50054
ENTRYPOINT ["dotnet", "ProjectService.dll"]
