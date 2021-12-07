using System.Linq;
using SharedDtos.Contracts;
using SharedDtos.Contracts.Generator;

var outputDirectory = args.FirstOrDefault() ?? "generated";

SchemaGenerator.ProduceSchema<MyDto>(outputDirectory);
