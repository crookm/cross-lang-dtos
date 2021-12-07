using System;
using System.IO;
using System.Text;
using NJsonSchema;

namespace SharedDtos.Contracts.Generator
{
    public static class SchemaGenerator
    {
        public static void ProduceSchema<TInput>(string outputDirectory)
        {
            if (!Directory.Exists(outputDirectory))
                Directory.CreateDirectory(outputDirectory);

            var outputFileName = Path.ChangeExtension(typeof(TInput).Name, "json");
            var outputPath = Path.Combine(outputDirectory, outputFileName);

            var schema = JsonSchema.FromType<TInput>().ToJson();

            File.WriteAllText(outputPath, schema, new UTF8Encoding(false));
            Console.WriteLine($"Generated JSON Schema for {typeof(TInput).Name} at: {outputPath}");
        }
    }
}