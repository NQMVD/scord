import React, {SVGProps} from 'react';

type IconProps = SVGProps<SVGSVGElement> & {
	secondaryfill?: string;
	strokewidth?: number;
	title?: string;
}

function Files({fill = 'currentColor', secondaryfill, title = 'badge 13', ...props}: IconProps) {
	secondaryfill = secondaryfill || fill;

	return (
		<svg height="24" style={{}} width="24" {...props} viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
	<title>{title}</title>
	<g fill="none">
		<path d="M8.40248 1C7.3938 1 6.42236 1.38108 5.68272 2.06693L2.28024 5.22196C1.46393 5.9789 1 7.04178 1 8.15503L1 14.5C1 16.433 2.567 18 4.5 18H12.5C14.433 18 16 16.433 16 14.5V4.5C16 2.567 14.433 1 12.5 1L8.40248 1Z" fill="url(#1752500502785-6928967_files_existing_0_u7sd7dn4k)" mask="url(#1752500502785-6928967_files_mask_412n3jmqd)" data-glass="origin"/>
		<path clipPath="url(#1752500502785-6928967_files_clipPath_5sqyfl1z9)" d="M8.40248 1C7.3938 1 6.42236 1.38108 5.68272 2.06693L2.28024 5.22196C1.46393 5.9789 1 7.04178 1 8.15503L1 14.5C1 16.433 2.567 18 4.5 18H12.5C14.433 18 16 16.433 16 14.5V4.5C16 2.567 14.433 1 12.5 1L8.40248 1Z" fill="url(#1752500502785-6928967_files_existing_0_u7sd7dn4k)" data-glass="clone"/>
		<path d="M15.5975 6C16.6062 6 17.5776 6.38108 18.3173 7.06693L21.7198 10.222C22.5361 10.9789 23 12.0418 23 13.155L23 19.5C23 21.433 21.433 23 19.5 23L11.5 23C9.567 23 8 21.433 8 19.5L8 9.5C8 7.567 9.56701 6 11.5 6L15.5975 6Z" fill="url(#1752500502785-6928967_files_existing_1_yc9h3ai2m)" data-glass="blur"/>
		<path d="M8 19.5V9.5C8 7.567 9.567 6 11.5 6H15.5977L15.7861 6.00488C16.7267 6.04926 17.624 6.4244 18.3174 7.06738L21.7197 10.2217C22.536 10.9786 23 12.042 23 13.1553V19.5L22.9951 19.6797C22.9046 21.4697 21.4696 22.9046 19.6797 22.9951L19.5 23V22.25C21.0188 22.25 22.25 21.0188 22.25 19.5V13.1553C22.25 12.2508 21.8732 11.3865 21.21 10.7715L17.8076 7.61719C17.2067 7.05997 16.4172 6.75004 15.5977 6.75H11.5C9.98122 6.75 8.75 7.98122 8.75 9.5V19.5C8.75 21.0188 9.98122 22.25 11.5 22.25V23L11.3203 22.9951C9.47083 22.9016 8 21.3727 8 19.5ZM19.5 22.25V23H11.5V22.25H19.5Z" fill="url(#1752500502785-6928967_files_existing_2_bhvt9x3j0)"/>
		<defs>
			<linearGradient id="1752500502785-6928967_files_existing_0_u7sd7dn4k" gradientUnits="userSpaceOnUse" x1="8.5" x2="8.5" y1="1" y2="18">
				<stop stopColor="#575757"/>
				<stop offset="1" stopColor="#1f1e1e"/>
			</linearGradient>
			<linearGradient id="1752500502785-6928967_files_existing_1_yc9h3ai2m" gradientUnits="userSpaceOnUse" x1="23" x2="8" y1="14.5" y2="14.5">
				<stop stopColor="#e3e3e559" stopOpacity=".6"/>
				<stop offset="1" stopColor="#bbbbc0af" stopOpacity=".6"/>
			</linearGradient>
			<linearGradient id="1752500502785-6928967_files_existing_2_bhvt9x3j0" gradientUnits="userSpaceOnUse" x1="15.5" x2="15.5" y1="6" y2="15.845">
				<stop stopColor="#ffffff99"/>
				<stop offset="1" stopColor="#ffffff99" stopOpacity="0"/>
			</linearGradient>
			<clipPath id="1752500502785-6928967_files_clipPath_5sqyfl1z9">
				<path d="M15.5975 6C16.6062 6 17.5776 6.38108 18.3173 7.06693L21.7198 10.222C22.5361 10.9789 23 12.0418 23 13.155L23 19.5C23 21.433 21.433 23 19.5 23L11.5 23C9.567 23 8 21.433 8 19.5L8 9.5C8 7.567 9.56701 6 11.5 6L15.5975 6Z" fill="url(#1752500502785-6928967_files_existing_1_yc9h3ai2m)"/>
			</clipPath>
			<mask id="1752500502785-6928967_files_mask_412n3jmqd">
				<rect height="100%" width="100%" fill="#FFF"/>
				<path d="M15.5975 6C16.6062 6 17.5776 6.38108 18.3173 7.06693L21.7198 10.222C22.5361 10.9789 23 12.0418 23 13.155L23 19.5C23 21.433 21.433 23 19.5 23L11.5 23C9.567 23 8 21.433 8 19.5L8 9.5C8 7.567 9.56701 6 11.5 6L15.5975 6Z" fill="#000"/>
			</mask>
		</defs>
	</g>
</svg>
	);
};

export default Files;