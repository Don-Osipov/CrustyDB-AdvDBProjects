## Summary of [Paper Title]

### Authors
- **Michael Freitag**, **Alfons Kemper**,**Thomas Neumann**  
- Published in: PVLDB, 15(11): 2797 - 2810, 2022.

### Overview
The paper centers around a hybrid database system, nearing in-memory performance without as many drawbacks. The authors propose a MVCC system that stores versioning information in memory, achieving high performance and more scalability than full in-memory systems.

### Key Contributions
1. **Problem Definition**: Current databses systems are either in-memory with high performance but size limitations, or disk-based, which have low performance.
2. **Proposed Methodology**: High-speed disks supplemented with in-memory versioning data, with lightweight fallback process for large write transactions.
3. **Results**: Up to 9.2x faster than postgres and 27.6x faster than a commercial disk-based database.
4. **Impact**: Shows the value of hybrid approches, allowing disk-based systems to get near in-memory performance will still retaining the scalability of disk-based
5. **Opportunities**: Only tests TATP/TPCC workloads through benchmarks. Could extend testing to include other, more diverse benchmarks. What happens if the database is spread across multiple servers?

### Methodology
Uses buffer manager to maintain mapping between logical data objects and in-memory versioning data, so that only the newest data version goes to disk.
Lightweight backup process for large transactions by storing simple flags in database pages instead of full version history
Get outcome by testing the system by comparing performance against other databases

### Results and Evaluation
The authors system achieves up to 9.2x faster than postgres and 27.6x faster than a commercial disk-based database.
Evaluations were performed using TATP and TPCC benchmarks with various load sizes, thread counts, and data sizes, showing that the system maintains high performance even when datasets are larger than RAM storage and during concurrent bulk tranasactions.

### Conclusion
Memory optimized disk-based systems can offer very good performance while maintaining the ability to handle large datasets. This suggests that it is possible next step for database system design.
### Key Takeaways
System design should keep in mind the most comomon workloads. In this case, the authors saw that most transactions were small and reads, so the system works great for those. Then they covered the less-common cases.
Limited growth in RAM size suggests that the issues of pure in-memory systems will not likely be resolved, and hybrid solutions will continue being relevant.
Its important to look at evaluation and benchmarks carefully. The authors say that they get 18.8x max performance over an pure in-memory system but the one they chose had lower performance than standard.
### Parts You Could Not Understand
The details of how the hybrid mutex implementation works for their locking mechanism
How can reads traverse version chains without latching and still maintaining correctness