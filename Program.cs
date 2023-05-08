using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;

namespace c_sharp_vs_rust_article
{
    class DataManager
    {
        private readonly ImmutableList<int> myPrivateData;
        public ImmutableList<int> GetData() => myPrivateData;

        public DataManager()
        {
            myPrivateData = ImmutableList.Create<int>(1, 2, 3, 4, 5);
        }
    }

    class Program
    {
        static void Main()
        {
            var dataManager = new DataManager();
            var data = dataManager.GetData();

            // here we do some processing with the data
            data.Add(6);
            data = data.Skip(2).Take(4).ToImmutableList();

            Console.WriteLine(string.Join(", ", data));
            Console.WriteLine(string.Join(", ", dataManager.GetData()));
        }
    }
}