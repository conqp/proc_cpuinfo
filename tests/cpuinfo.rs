use proc_cpuinfo::CpuInfo;
use std::str::FromStr;

const CPU_INFO: &str = "processor	: 0
vendor_id	: GenuineIntel
cpu family	: 6
model		: 151
model name	: 12th Gen Intel(R) Core(TM) i5-12400
stepping	: 5
microcode	: 0x2c
cpu MHz		: 2500.000
cache size	: 18432 KB
physical id	: 0
siblings	: 12
core id		: 0
cpu cores	: 6
apicid		: 0
initial apicid	: 0
fpu		: yes
fpu_exception	: yes
cpuid level	: 32
wp		: yes
flags		: fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx fxsr sse sse2 ss ht tm pbe syscall nx pdpe1gb rdtscp lm constant_tsc art arch_perfmon pebs bts rep_good nopl xtopology nonstop_tsc cpuid aperfmperf tsc_known_freq pni pclmulqdq dtes64 monitor ds_cpl vmx est tm2 ssse3 sdbg fma cx16 xtpr pdcm sse4_1 sse4_2 x2apic movbe popcnt tsc_deadline_timer aes xsave avx f16c rdrand lahf_lm abm 3dnowprefetch cpuid_fault epb cat_l2 cdp_l2 ssbd ibrs ibpb stibp ibrs_enhanced tpr_shadow flexpriority ept vpid ept_ad fsgsbase tsc_adjust bmi1 avx2 smep bmi2 erms invpcid rdt_a rdseed adx smap clflushopt clwb intel_pt sha_ni xsaveopt xsavec xgetbv1 xsaves split_lock_detect avx_vnni dtherm ida arat pln pts hwp hwp_notify hwp_act_window hwp_epp hwp_pkg_req hfi vnmi umip pku ospke waitpkg gfni vaes vpclmulqdq rdpid movdiri movdir64b fsrm md_clear serialize arch_lbr ibt flush_l1d arch_capabilities
vmx flags	: vnmi preemption_timer posted_intr invvpid ept_x_only ept_ad ept_1gb flexpriority apicv tsc_offset vtpr mtf vapic ept vpid unrestricted_guest vapic_reg vid ple shadow_vmcs pml ept_mode_based_exec tsc_scaling usr_wait_pause
bugs		: spectre_v1 spectre_v2 spec_store_bypass swapgs eibrs_pbrsb
bogomips	: 4993.00
clflush size	: 64
cache_alignment	: 64
address sizes	: 39 bits physical, 48 bits virtual
power management:

processor	: 1
vendor_id	: GenuineIntel
cpu family	: 6
model		: 151
model name	: 12th Gen Intel(R) Core(TM) i5-12400
stepping	: 5
microcode	: 0x2c
cpu MHz		: 2500.000
cache size	: 18432 KB
physical id	: 0
siblings	: 12
core id		: 0
cpu cores	: 6
apicid		: 1
initial apicid	: 1
fpu		: yes
fpu_exception	: yes
cpuid level	: 32
wp		: yes
flags		: fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx fxsr sse sse2 ss ht tm pbe syscall nx pdpe1gb rdtscp lm constant_tsc art arch_perfmon pebs bts rep_good nopl xtopology nonstop_tsc cpuid aperfmperf tsc_known_freq pni pclmulqdq dtes64 monitor ds_cpl vmx est tm2 ssse3 sdbg fma cx16 xtpr pdcm sse4_1 sse4_2 x2apic movbe popcnt tsc_deadline_timer aes xsave avx f16c rdrand lahf_lm abm 3dnowprefetch cpuid_fault epb cat_l2 cdp_l2 ssbd ibrs ibpb stibp ibrs_enhanced tpr_shadow flexpriority ept vpid ept_ad fsgsbase tsc_adjust bmi1 avx2 smep bmi2 erms invpcid rdt_a rdseed adx smap clflushopt clwb intel_pt sha_ni xsaveopt xsavec xgetbv1 xsaves split_lock_detect avx_vnni dtherm ida arat pln pts hwp hwp_notify hwp_act_window hwp_epp hwp_pkg_req hfi vnmi umip pku ospke waitpkg gfni vaes vpclmulqdq rdpid movdiri movdir64b fsrm md_clear serialize arch_lbr ibt flush_l1d arch_capabilities
vmx flags	: vnmi preemption_timer posted_intr invvpid ept_x_only ept_ad ept_1gb flexpriority apicv tsc_offset vtpr mtf vapic ept vpid unrestricted_guest vapic_reg vid ple shadow_vmcs pml ept_mode_based_exec tsc_scaling usr_wait_pause
bugs		: spectre_v1 spectre_v2 spec_store_bypass swapgs eibrs_pbrsb
bogomips	: 4993.00
clflush size	: 64
cache_alignment	: 64
address sizes	: 39 bits physical, 48 bits virtual
power management:

processor	: 2
vendor_id	: GenuineIntel
cpu family	: 6
model		: 151
model name	: 12th Gen Intel(R) Core(TM) i5-12400
stepping	: 5
microcode	: 0x2c
cpu MHz		: 2500.000
cache size	: 18432 KB
physical id	: 0
siblings	: 12
core id		: 1
cpu cores	: 6
apicid		: 2
initial apicid	: 2
fpu		: yes
fpu_exception	: yes
cpuid level	: 32
wp		: yes
flags		: fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx fxsr sse sse2 ss ht tm pbe syscall nx pdpe1gb rdtscp lm constant_tsc art arch_perfmon pebs bts rep_good nopl xtopology nonstop_tsc cpuid aperfmperf tsc_known_freq pni pclmulqdq dtes64 monitor ds_cpl vmx est tm2 ssse3 sdbg fma cx16 xtpr pdcm sse4_1 sse4_2 x2apic movbe popcnt tsc_deadline_timer aes xsave avx f16c rdrand lahf_lm abm 3dnowprefetch cpuid_fault epb cat_l2 cdp_l2 ssbd ibrs ibpb stibp ibrs_enhanced tpr_shadow flexpriority ept vpid ept_ad fsgsbase tsc_adjust bmi1 avx2 smep bmi2 erms invpcid rdt_a rdseed adx smap clflushopt clwb intel_pt sha_ni xsaveopt xsavec xgetbv1 xsaves split_lock_detect avx_vnni dtherm ida arat pln pts hwp hwp_notify hwp_act_window hwp_epp hwp_pkg_req hfi vnmi umip pku ospke waitpkg gfni vaes vpclmulqdq rdpid movdiri movdir64b fsrm md_clear serialize arch_lbr ibt flush_l1d arch_capabilities
vmx flags	: vnmi preemption_timer posted_intr invvpid ept_x_only ept_ad ept_1gb flexpriority apicv tsc_offset vtpr mtf vapic ept vpid unrestricted_guest vapic_reg vid ple shadow_vmcs pml ept_mode_based_exec tsc_scaling usr_wait_pause
bugs		: spectre_v1 spectre_v2 spec_store_bypass swapgs eibrs_pbrsb
bogomips	: 4993.00
clflush size	: 64
cache_alignment	: 64
address sizes	: 39 bits physical, 48 bits virtual
power management:

processor	: 3
vendor_id	: GenuineIntel
cpu family	: 6
model		: 151
model name	: 12th Gen Intel(R) Core(TM) i5-12400
stepping	: 5
microcode	: 0x2c
cpu MHz		: 2500.000
cache size	: 18432 KB
physical id	: 0
siblings	: 12
core id		: 1
cpu cores	: 6
apicid		: 3
initial apicid	: 3
fpu		: yes
fpu_exception	: yes
cpuid level	: 32
wp		: yes
flags		: fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx fxsr sse sse2 ss ht tm pbe syscall nx pdpe1gb rdtscp lm constant_tsc art arch_perfmon pebs bts rep_good nopl xtopology nonstop_tsc cpuid aperfmperf tsc_known_freq pni pclmulqdq dtes64 monitor ds_cpl vmx est tm2 ssse3 sdbg fma cx16 xtpr pdcm sse4_1 sse4_2 x2apic movbe popcnt tsc_deadline_timer aes xsave avx f16c rdrand lahf_lm abm 3dnowprefetch cpuid_fault epb cat_l2 cdp_l2 ssbd ibrs ibpb stibp ibrs_enhanced tpr_shadow flexpriority ept vpid ept_ad fsgsbase tsc_adjust bmi1 avx2 smep bmi2 erms invpcid rdt_a rdseed adx smap clflushopt clwb intel_pt sha_ni xsaveopt xsavec xgetbv1 xsaves split_lock_detect avx_vnni dtherm ida arat pln pts hwp hwp_notify hwp_act_window hwp_epp hwp_pkg_req hfi vnmi umip pku ospke waitpkg gfni vaes vpclmulqdq rdpid movdiri movdir64b fsrm md_clear serialize arch_lbr ibt flush_l1d arch_capabilities
vmx flags	: vnmi preemption_timer posted_intr invvpid ept_x_only ept_ad ept_1gb flexpriority apicv tsc_offset vtpr mtf vapic ept vpid unrestricted_guest vapic_reg vid ple shadow_vmcs pml ept_mode_based_exec tsc_scaling usr_wait_pause
bugs		: spectre_v1 spectre_v2 spec_store_bypass swapgs eibrs_pbrsb
bogomips	: 4993.00
clflush size	: 64
cache_alignment	: 64
address sizes	: 39 bits physical, 48 bits virtual
power management:

processor	: 4
vendor_id	: GenuineIntel
cpu family	: 6
model		: 151
model name	: 12th Gen Intel(R) Core(TM) i5-12400
stepping	: 5
microcode	: 0x2c
cpu MHz		: 800.116
cache size	: 18432 KB
physical id	: 0
siblings	: 12
core id		: 2
cpu cores	: 6
apicid		: 4
initial apicid	: 4
fpu		: yes
fpu_exception	: yes
cpuid level	: 32
wp		: yes
flags		: fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx fxsr sse sse2 ss ht tm pbe syscall nx pdpe1gb rdtscp lm constant_tsc art arch_perfmon pebs bts rep_good nopl xtopology nonstop_tsc cpuid aperfmperf tsc_known_freq pni pclmulqdq dtes64 monitor ds_cpl vmx est tm2 ssse3 sdbg fma cx16 xtpr pdcm sse4_1 sse4_2 x2apic movbe popcnt tsc_deadline_timer aes xsave avx f16c rdrand lahf_lm abm 3dnowprefetch cpuid_fault epb cat_l2 cdp_l2 ssbd ibrs ibpb stibp ibrs_enhanced tpr_shadow flexpriority ept vpid ept_ad fsgsbase tsc_adjust bmi1 avx2 smep bmi2 erms invpcid rdt_a rdseed adx smap clflushopt clwb intel_pt sha_ni xsaveopt xsavec xgetbv1 xsaves split_lock_detect avx_vnni dtherm ida arat pln pts hwp hwp_notify hwp_act_window hwp_epp hwp_pkg_req hfi vnmi umip pku ospke waitpkg gfni vaes vpclmulqdq rdpid movdiri movdir64b fsrm md_clear serialize arch_lbr ibt flush_l1d arch_capabilities
vmx flags	: vnmi preemption_timer posted_intr invvpid ept_x_only ept_ad ept_1gb flexpriority apicv tsc_offset vtpr mtf vapic ept vpid unrestricted_guest vapic_reg vid ple shadow_vmcs pml ept_mode_based_exec tsc_scaling usr_wait_pause
bugs		: spectre_v1 spectre_v2 spec_store_bypass swapgs eibrs_pbrsb
bogomips	: 4993.00
clflush size	: 64
cache_alignment	: 64
address sizes	: 39 bits physical, 48 bits virtual
power management:

processor	: 5
vendor_id	: GenuineIntel
cpu family	: 6
model		: 151
model name	: 12th Gen Intel(R) Core(TM) i5-12400
stepping	: 5
microcode	: 0x2c
cpu MHz		: 2500.000
cache size	: 18432 KB
physical id	: 0
siblings	: 12
core id		: 2
cpu cores	: 6
apicid		: 5
initial apicid	: 5
fpu		: yes
fpu_exception	: yes
cpuid level	: 32
wp		: yes
flags		: fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx fxsr sse sse2 ss ht tm pbe syscall nx pdpe1gb rdtscp lm constant_tsc art arch_perfmon pebs bts rep_good nopl xtopology nonstop_tsc cpuid aperfmperf tsc_known_freq pni pclmulqdq dtes64 monitor ds_cpl vmx est tm2 ssse3 sdbg fma cx16 xtpr pdcm sse4_1 sse4_2 x2apic movbe popcnt tsc_deadline_timer aes xsave avx f16c rdrand lahf_lm abm 3dnowprefetch cpuid_fault epb cat_l2 cdp_l2 ssbd ibrs ibpb stibp ibrs_enhanced tpr_shadow flexpriority ept vpid ept_ad fsgsbase tsc_adjust bmi1 avx2 smep bmi2 erms invpcid rdt_a rdseed adx smap clflushopt clwb intel_pt sha_ni xsaveopt xsavec xgetbv1 xsaves split_lock_detect avx_vnni dtherm ida arat pln pts hwp hwp_notify hwp_act_window hwp_epp hwp_pkg_req hfi vnmi umip pku ospke waitpkg gfni vaes vpclmulqdq rdpid movdiri movdir64b fsrm md_clear serialize arch_lbr ibt flush_l1d arch_capabilities
vmx flags	: vnmi preemption_timer posted_intr invvpid ept_x_only ept_ad ept_1gb flexpriority apicv tsc_offset vtpr mtf vapic ept vpid unrestricted_guest vapic_reg vid ple shadow_vmcs pml ept_mode_based_exec tsc_scaling usr_wait_pause
bugs		: spectre_v1 spectre_v2 spec_store_bypass swapgs eibrs_pbrsb
bogomips	: 4993.00
clflush size	: 64
cache_alignment	: 64
address sizes	: 39 bits physical, 48 bits virtual
power management:

processor	: 6
vendor_id	: GenuineIntel
cpu family	: 6
model		: 151
model name	: 12th Gen Intel(R) Core(TM) i5-12400
stepping	: 5
microcode	: 0x2c
cpu MHz		: 800.022
cache size	: 18432 KB
physical id	: 0
siblings	: 12
core id		: 3
cpu cores	: 6
apicid		: 6
initial apicid	: 6
fpu		: yes
fpu_exception	: yes
cpuid level	: 32
wp		: yes
flags		: fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx fxsr sse sse2 ss ht tm pbe syscall nx pdpe1gb rdtscp lm constant_tsc art arch_perfmon pebs bts rep_good nopl xtopology nonstop_tsc cpuid aperfmperf tsc_known_freq pni pclmulqdq dtes64 monitor ds_cpl vmx est tm2 ssse3 sdbg fma cx16 xtpr pdcm sse4_1 sse4_2 x2apic movbe popcnt tsc_deadline_timer aes xsave avx f16c rdrand lahf_lm abm 3dnowprefetch cpuid_fault epb cat_l2 cdp_l2 ssbd ibrs ibpb stibp ibrs_enhanced tpr_shadow flexpriority ept vpid ept_ad fsgsbase tsc_adjust bmi1 avx2 smep bmi2 erms invpcid rdt_a rdseed adx smap clflushopt clwb intel_pt sha_ni xsaveopt xsavec xgetbv1 xsaves split_lock_detect avx_vnni dtherm ida arat pln pts hwp hwp_notify hwp_act_window hwp_epp hwp_pkg_req hfi vnmi umip pku ospke waitpkg gfni vaes vpclmulqdq rdpid movdiri movdir64b fsrm md_clear serialize arch_lbr ibt flush_l1d arch_capabilities
vmx flags	: vnmi preemption_timer posted_intr invvpid ept_x_only ept_ad ept_1gb flexpriority apicv tsc_offset vtpr mtf vapic ept vpid unrestricted_guest vapic_reg vid ple shadow_vmcs pml ept_mode_based_exec tsc_scaling usr_wait_pause
bugs		: spectre_v1 spectre_v2 spec_store_bypass swapgs eibrs_pbrsb
bogomips	: 4993.00
clflush size	: 64
cache_alignment	: 64
address sizes	: 39 bits physical, 48 bits virtual
power management:

processor	: 7
vendor_id	: GenuineIntel
cpu family	: 6
model		: 151
model name	: 12th Gen Intel(R) Core(TM) i5-12400
stepping	: 5
microcode	: 0x2c
cpu MHz		: 2500.000
cache size	: 18432 KB
physical id	: 0
siblings	: 12
core id		: 3
cpu cores	: 6
apicid		: 7
initial apicid	: 7
fpu		: yes
fpu_exception	: yes
cpuid level	: 32
wp		: yes
flags		: fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx fxsr sse sse2 ss ht tm pbe syscall nx pdpe1gb rdtscp lm constant_tsc art arch_perfmon pebs bts rep_good nopl xtopology nonstop_tsc cpuid aperfmperf tsc_known_freq pni pclmulqdq dtes64 monitor ds_cpl vmx est tm2 ssse3 sdbg fma cx16 xtpr pdcm sse4_1 sse4_2 x2apic movbe popcnt tsc_deadline_timer aes xsave avx f16c rdrand lahf_lm abm 3dnowprefetch cpuid_fault epb cat_l2 cdp_l2 ssbd ibrs ibpb stibp ibrs_enhanced tpr_shadow flexpriority ept vpid ept_ad fsgsbase tsc_adjust bmi1 avx2 smep bmi2 erms invpcid rdt_a rdseed adx smap clflushopt clwb intel_pt sha_ni xsaveopt xsavec xgetbv1 xsaves split_lock_detect avx_vnni dtherm ida arat pln pts hwp hwp_notify hwp_act_window hwp_epp hwp_pkg_req hfi vnmi umip pku ospke waitpkg gfni vaes vpclmulqdq rdpid movdiri movdir64b fsrm md_clear serialize arch_lbr ibt flush_l1d arch_capabilities
vmx flags	: vnmi preemption_timer posted_intr invvpid ept_x_only ept_ad ept_1gb flexpriority apicv tsc_offset vtpr mtf vapic ept vpid unrestricted_guest vapic_reg vid ple shadow_vmcs pml ept_mode_based_exec tsc_scaling usr_wait_pause
bugs		: spectre_v1 spectre_v2 spec_store_bypass swapgs eibrs_pbrsb
bogomips	: 4993.00
clflush size	: 64
cache_alignment	: 64
address sizes	: 39 bits physical, 48 bits virtual
power management:

processor	: 8
vendor_id	: GenuineIntel
cpu family	: 6
model		: 151
model name	: 12th Gen Intel(R) Core(TM) i5-12400
stepping	: 5
microcode	: 0x2c
cpu MHz		: 2500.000
cache size	: 18432 KB
physical id	: 0
siblings	: 12
core id		: 4
cpu cores	: 6
apicid		: 8
initial apicid	: 8
fpu		: yes
fpu_exception	: yes
cpuid level	: 32
wp		: yes
flags		: fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx fxsr sse sse2 ss ht tm pbe syscall nx pdpe1gb rdtscp lm constant_tsc art arch_perfmon pebs bts rep_good nopl xtopology nonstop_tsc cpuid aperfmperf tsc_known_freq pni pclmulqdq dtes64 monitor ds_cpl vmx est tm2 ssse3 sdbg fma cx16 xtpr pdcm sse4_1 sse4_2 x2apic movbe popcnt tsc_deadline_timer aes xsave avx f16c rdrand lahf_lm abm 3dnowprefetch cpuid_fault epb cat_l2 cdp_l2 ssbd ibrs ibpb stibp ibrs_enhanced tpr_shadow flexpriority ept vpid ept_ad fsgsbase tsc_adjust bmi1 avx2 smep bmi2 erms invpcid rdt_a rdseed adx smap clflushopt clwb intel_pt sha_ni xsaveopt xsavec xgetbv1 xsaves split_lock_detect avx_vnni dtherm ida arat pln pts hwp hwp_notify hwp_act_window hwp_epp hwp_pkg_req hfi vnmi umip pku ospke waitpkg gfni vaes vpclmulqdq rdpid movdiri movdir64b fsrm md_clear serialize arch_lbr ibt flush_l1d arch_capabilities
vmx flags	: vnmi preemption_timer posted_intr invvpid ept_x_only ept_ad ept_1gb flexpriority apicv tsc_offset vtpr mtf vapic ept vpid unrestricted_guest vapic_reg vid ple shadow_vmcs pml ept_mode_based_exec tsc_scaling usr_wait_pause
bugs		: spectre_v1 spectre_v2 spec_store_bypass swapgs eibrs_pbrsb
bogomips	: 4993.00
clflush size	: 64
cache_alignment	: 64
address sizes	: 39 bits physical, 48 bits virtual
power management:

processor	: 9
vendor_id	: GenuineIntel
cpu family	: 6
model		: 151
model name	: 12th Gen Intel(R) Core(TM) i5-12400
stepping	: 5
microcode	: 0x2c
cpu MHz		: 2500.000
cache size	: 18432 KB
physical id	: 0
siblings	: 12
core id		: 4
cpu cores	: 6
apicid		: 9
initial apicid	: 9
fpu		: yes
fpu_exception	: yes
cpuid level	: 32
wp		: yes
flags		: fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx fxsr sse sse2 ss ht tm pbe syscall nx pdpe1gb rdtscp lm constant_tsc art arch_perfmon pebs bts rep_good nopl xtopology nonstop_tsc cpuid aperfmperf tsc_known_freq pni pclmulqdq dtes64 monitor ds_cpl vmx est tm2 ssse3 sdbg fma cx16 xtpr pdcm sse4_1 sse4_2 x2apic movbe popcnt tsc_deadline_timer aes xsave avx f16c rdrand lahf_lm abm 3dnowprefetch cpuid_fault epb cat_l2 cdp_l2 ssbd ibrs ibpb stibp ibrs_enhanced tpr_shadow flexpriority ept vpid ept_ad fsgsbase tsc_adjust bmi1 avx2 smep bmi2 erms invpcid rdt_a rdseed adx smap clflushopt clwb intel_pt sha_ni xsaveopt xsavec xgetbv1 xsaves split_lock_detect avx_vnni dtherm ida arat pln pts hwp hwp_notify hwp_act_window hwp_epp hwp_pkg_req hfi vnmi umip pku ospke waitpkg gfni vaes vpclmulqdq rdpid movdiri movdir64b fsrm md_clear serialize arch_lbr ibt flush_l1d arch_capabilities
vmx flags	: vnmi preemption_timer posted_intr invvpid ept_x_only ept_ad ept_1gb flexpriority apicv tsc_offset vtpr mtf vapic ept vpid unrestricted_guest vapic_reg vid ple shadow_vmcs pml ept_mode_based_exec tsc_scaling usr_wait_pause
bugs		: spectre_v1 spectre_v2 spec_store_bypass swapgs eibrs_pbrsb
bogomips	: 4993.00
clflush size	: 64
cache_alignment	: 64
address sizes	: 39 bits physical, 48 bits virtual
power management:

processor	: 10
vendor_id	: GenuineIntel
cpu family	: 6
model		: 151
model name	: 12th Gen Intel(R) Core(TM) i5-12400
stepping	: 5
microcode	: 0x2c
cpu MHz		: 2500.000
cache size	: 18432 KB
physical id	: 0
siblings	: 12
core id		: 5
cpu cores	: 6
apicid		: 10
initial apicid	: 10
fpu		: yes
fpu_exception	: yes
cpuid level	: 32
wp		: yes
flags		: fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx fxsr sse sse2 ss ht tm pbe syscall nx pdpe1gb rdtscp lm constant_tsc art arch_perfmon pebs bts rep_good nopl xtopology nonstop_tsc cpuid aperfmperf tsc_known_freq pni pclmulqdq dtes64 monitor ds_cpl vmx est tm2 ssse3 sdbg fma cx16 xtpr pdcm sse4_1 sse4_2 x2apic movbe popcnt tsc_deadline_timer aes xsave avx f16c rdrand lahf_lm abm 3dnowprefetch cpuid_fault epb cat_l2 cdp_l2 ssbd ibrs ibpb stibp ibrs_enhanced tpr_shadow flexpriority ept vpid ept_ad fsgsbase tsc_adjust bmi1 avx2 smep bmi2 erms invpcid rdt_a rdseed adx smap clflushopt clwb intel_pt sha_ni xsaveopt xsavec xgetbv1 xsaves split_lock_detect avx_vnni dtherm ida arat pln pts hwp hwp_notify hwp_act_window hwp_epp hwp_pkg_req hfi vnmi umip pku ospke waitpkg gfni vaes vpclmulqdq rdpid movdiri movdir64b fsrm md_clear serialize arch_lbr ibt flush_l1d arch_capabilities
vmx flags	: vnmi preemption_timer posted_intr invvpid ept_x_only ept_ad ept_1gb flexpriority apicv tsc_offset vtpr mtf vapic ept vpid unrestricted_guest vapic_reg vid ple shadow_vmcs pml ept_mode_based_exec tsc_scaling usr_wait_pause
bugs		: spectre_v1 spectre_v2 spec_store_bypass swapgs eibrs_pbrsb
bogomips	: 4993.00
clflush size	: 64
cache_alignment	: 64
address sizes	: 39 bits physical, 48 bits virtual
power management:

processor	: 11
vendor_id	: GenuineIntel
cpu family	: 6
model		: 151
model name	: 12th Gen Intel(R) Core(TM) i5-12400
stepping	: 5
microcode	: 0x2c
cpu MHz		: 800.054
cache size	: 18432 KB
physical id	: 0
siblings	: 12
core id		: 5
cpu cores	: 6
apicid		: 11
initial apicid	: 11
fpu		: yes
fpu_exception	: yes
cpuid level	: 32
wp		: yes
flags		: fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx fxsr sse sse2 ss ht tm pbe syscall nx pdpe1gb rdtscp lm constant_tsc art arch_perfmon pebs bts rep_good nopl xtopology nonstop_tsc cpuid aperfmperf tsc_known_freq pni pclmulqdq dtes64 monitor ds_cpl vmx est tm2 ssse3 sdbg fma cx16 xtpr pdcm sse4_1 sse4_2 x2apic movbe popcnt tsc_deadline_timer aes xsave avx f16c rdrand lahf_lm abm 3dnowprefetch cpuid_fault epb cat_l2 cdp_l2 ssbd ibrs ibpb stibp ibrs_enhanced tpr_shadow flexpriority ept vpid ept_ad fsgsbase tsc_adjust bmi1 avx2 smep bmi2 erms invpcid rdt_a rdseed adx smap clflushopt clwb intel_pt sha_ni xsaveopt xsavec xgetbv1 xsaves split_lock_detect avx_vnni dtherm ida arat pln pts hwp hwp_notify hwp_act_window hwp_epp hwp_pkg_req hfi vnmi umip pku ospke waitpkg gfni vaes vpclmulqdq rdpid movdiri movdir64b fsrm md_clear serialize arch_lbr ibt flush_l1d arch_capabilities
vmx flags	: vnmi preemption_timer posted_intr invvpid ept_x_only ept_ad ept_1gb flexpriority apicv tsc_offset vtpr mtf vapic ept vpid unrestricted_guest vapic_reg vid ple shadow_vmcs pml ept_mode_based_exec tsc_scaling usr_wait_pause
bugs		: spectre_v1 spectre_v2 spec_store_bypass swapgs eibrs_pbrsb
bogomips	: 4993.00
clflush size	: 64
cache_alignment	: 64
address sizes	: 39 bits physical, 48 bits virtual
power management:

";
const FREQUENCIES: [f32; 12] = [
    2500.0, 2500.0, 2500.0, 2500.0, 800.116, 2500.0, 800.022, 2500.0, 2500.0, 2500.0, 2500.0,
    800.054,
];
const CORE_IDS: [usize; 12] = [0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5];

#[allow(clippy::unwrap_used)]
#[test]
fn test_cpu_count() {
    assert_eq!(CpuInfo::from_str(CPU_INFO).unwrap().iter().count(), 12);
}

#[allow(clippy::unwrap_used)]
#[test]
fn test_processor() {
    for (index, cpu) in CpuInfo::from_str(CPU_INFO).unwrap().iter().enumerate() {
        assert_eq!(cpu.processor(), Some(index));
    }
}

#[allow(clippy::unwrap_used)]
#[test]
fn test_vendor_id() {
    for cpu in CpuInfo::from_str(CPU_INFO).unwrap().iter() {
        assert_eq!(cpu.vendor_id(), Some("GenuineIntel"));
    }
}

#[allow(clippy::unwrap_used)]
#[test]
fn test_cpu_family() {
    for cpu in CpuInfo::from_str(CPU_INFO).unwrap().iter() {
        assert_eq!(cpu.cpu_family(), Some(6));
    }
}

#[allow(clippy::unwrap_used)]
#[test]
fn test_model() {
    for cpu in CpuInfo::from_str(CPU_INFO).unwrap().iter() {
        assert_eq!(cpu.model(), Some(151));
    }
}

#[allow(clippy::unwrap_used)]
#[test]
fn test_model_name() {
    for cpu in CpuInfo::from_str(CPU_INFO).unwrap().iter() {
        assert_eq!(
            cpu.model_name(),
            Some("12th Gen Intel(R) Core(TM) i5-12400")
        );
    }
}

#[allow(clippy::unwrap_used)]
#[test]
fn test_stepping() {
    for cpu in CpuInfo::from_str(CPU_INFO).unwrap().iter() {
        assert_eq!(cpu.stepping(), Some(5));
    }
}

#[allow(clippy::unwrap_used)]
#[test]
fn test_microcode() {
    for cpu in CpuInfo::from_str(CPU_INFO).unwrap().iter() {
        assert_eq!(cpu.microcode(), Some(0x2c));
    }
}

#[allow(clippy::unwrap_used)]
#[test]
fn test_cpu_mhz() {
    for (cpu, freq) in CpuInfo::from_str(CPU_INFO).unwrap().iter().zip(FREQUENCIES) {
        assert_eq!(cpu.cpu_mhz(), Some(freq));
    }
}

#[allow(clippy::unwrap_used)]
#[test]
fn test_cache_size() {
    for cpu in CpuInfo::from_str(CPU_INFO).unwrap().iter() {
        assert_eq!(cpu.cache_size(), Some(18432 * 1024));
    }
}

#[allow(clippy::unwrap_used)]
#[test]
fn test_physical_id() {
    for cpu in CpuInfo::from_str(CPU_INFO).unwrap().iter() {
        assert_eq!(cpu.physical_id(), Some(0));
    }
}

#[allow(clippy::unwrap_used)]
#[test]
fn test_siblings() {
    for cpu in CpuInfo::from_str(CPU_INFO).unwrap().iter() {
        assert_eq!(cpu.siblings(), Some(12));
    }
}

#[allow(clippy::unwrap_used)]
#[test]
fn test_core_id() {
    for (cpu, core_id) in CpuInfo::from_str(CPU_INFO).unwrap().iter().zip(CORE_IDS) {
        assert_eq!(cpu.core_id(), Some(core_id));
    }
}

#[allow(clippy::unwrap_used)]
#[test]
fn test_cpu_cores() {
    for cpu in CpuInfo::from_str(CPU_INFO).unwrap().iter() {
        assert_eq!(cpu.cpu_cores(), Some(6));
    }
}

#[allow(clippy::unwrap_used)]
#[test]
fn test_apicid() {
    for (index, cpu) in CpuInfo::from_str(CPU_INFO).unwrap().iter().enumerate() {
        assert_eq!(cpu.apicid(), Some(index));
    }
}

#[allow(clippy::unwrap_used)]
#[test]
fn test_initial_apicid() {
    for (index, cpu) in CpuInfo::from_str(CPU_INFO).unwrap().iter().enumerate() {
        assert_eq!(cpu.initial_apicid(), Some(index));
    }
}

#[allow(clippy::unwrap_used)]
#[test]
fn test_fpu() {
    for cpu in CpuInfo::from_str(CPU_INFO).unwrap().iter() {
        assert_eq!(cpu.fpu(), Some(true));
    }
}

#[allow(clippy::unwrap_used)]
#[test]
fn test_fpu_exception() {
    for cpu in CpuInfo::from_str(CPU_INFO).unwrap().iter() {
        assert_eq!(cpu.fpu_exception(), Some(true));
    }
}

#[allow(clippy::unwrap_used)]
#[test]
fn test_cpuid_level() {
    for cpu in CpuInfo::from_str(CPU_INFO).unwrap().iter() {
        assert_eq!(cpu.cpuid_level(), Some(32));
    }
}

#[allow(clippy::unwrap_used)]
#[test]
fn test_wp() {
    for cpu in CpuInfo::from_str(CPU_INFO).unwrap().iter() {
        assert_eq!(cpu.wp(), Some(true));
    }
}

#[allow(clippy::unwrap_used)]
#[test]
fn test_flags() {
    for cpu in CpuInfo::from_str(CPU_INFO).unwrap().iter() {
        assert!(cpu.flags().contains("mmx"));
    }
}

#[allow(clippy::unwrap_used)]
#[test]
fn test_vmx_flags() {
    for cpu in CpuInfo::from_str(CPU_INFO).unwrap().iter() {
        assert!(cpu.vmx_flags().contains("flexpriority"));
    }
}

#[allow(clippy::unwrap_used)]
#[test]
fn test_bugs() {
    for cpu in CpuInfo::from_str(CPU_INFO).unwrap().iter() {
        assert!(cpu.bugs().contains("spectre_v2"));
    }
}

#[allow(clippy::unwrap_used)]
#[test]
fn test_bogomips() {
    for cpu in CpuInfo::from_str(CPU_INFO).unwrap().iter() {
        assert_eq!(cpu.bogomips(), Some(4993.00));
    }
}

#[allow(clippy::unwrap_used)]
#[test]
fn test_clflush_size() {
    for cpu in CpuInfo::from_str(CPU_INFO).unwrap().iter() {
        assert_eq!(cpu.clflush_size(), Some(64));
    }
}

#[allow(clippy::unwrap_used)]
#[test]
fn test_cache_alignment() {
    for cpu in CpuInfo::from_str(CPU_INFO).unwrap().iter() {
        assert_eq!(cpu.cache_alignment(), Some(64));
    }
}

#[allow(clippy::unwrap_used)]
#[test]
fn test_address_sizes() {
    for cpu in CpuInfo::from_str(CPU_INFO).unwrap().iter() {
        assert_eq!(cpu.address_sizes(), Some((39, 48)));
    }
}

#[allow(clippy::unwrap_used)]
#[test]
fn test_power_management() {
    for cpu in CpuInfo::from_str(CPU_INFO).unwrap().iter() {
        assert_eq!(cpu.power_management(), Some(""));
    }
}
