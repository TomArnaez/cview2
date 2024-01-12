const Panel = ({ title, children }) => {
    return (
      <div className="bg-secondary p-4 rounded-lg shadow-md">
        {title && <h2 className="text-text font-semibold mb-2">{title}</h2>}
        {children}
      </div>
    );
  };

const Setting = ({ label, children }) => {
return (
    <div className="mb-4 last:mb-0">
    {label && <label className="text-textSecondary block mb-1">{label}</label>}
    {children}
    </div>
);
};